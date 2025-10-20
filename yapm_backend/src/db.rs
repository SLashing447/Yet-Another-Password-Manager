use anyhow::{Ok, Result, anyhow, bail};
use rusqlite::{Connection, Result as SqlResult, params};
// use rusqlite::Result;
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    thread,
};

use crate::{
    crypto,
    types::{Card, Vault, VaultMd},
};

pub struct Database {
    conn: Arc<Mutex<Connection>>,
    master_key: String,
    vault_keys: Arc<Mutex<HashMap<i64, String>>>,
    vaults_fetched: Arc<AtomicU64>,
}

impl Database {
    const KEY_ITR: u32 = 130;

    pub fn new(path: &str, password: &str) -> Result<Self> {
        let conn = Connection::open(path).unwrap();
        conn.pragma_update(None, "key", &password)
            .map_err(|e| anyhow!("Wrong Profile Password : {}", e))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS connections (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  vault_id INTEGER NOT NULL,
                  provider TEXT NOT NULL,
                  data BLOB NOT NULL,
                  nonce BLOB NOT NULL,
                  salt BLOB NOT NULL,
                  FOREIGN KEY (vault_id) REFERENCES vaults(id)
                )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS vaultsMd (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  vault_id INTEGER NOT NULL,
                  data BLOB NOT NULL,
                  nonce BLOB NOT NULL,
                  salt BLOB NOT NULL,
                  FOREIGN KEY (vault_id) REFERENCES vaults(id)
                )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS vaults (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,          
                    name TEXT NOT NULL,                     
                    desc TEXT,                               
                    created_at INTEGER NOT NULL,              
                    last_accessed INTEGER NOT NULL,           
                    img BLOB                               
                );",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            master_key: password.into(),
            vault_keys: Arc::new(Mutex::new(HashMap::new())),
            vaults_fetched: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Insert a Vault    
    /// Using `Struct Vault` no password -> db must be opend
    pub fn insert_vault(&self, vault: Vault, password: String) -> Result<i64> {
        let conn1 = self.conn.clone();
        let mk = self.master_key.clone();

        let handle = thread::spawn(move || {
            let conn = conn1.lock().unwrap();
            let components: Vec<&str> = vec![&mk, &password];

            conn.execute(
                "INSERT INTO vaults (name, desc, created_at, last_accessed, img) VALUES ( ?, ?, ?, ?, ?)",
                params![
                    vault.name,
                    vault.desc,
                    vault.created_at as i64,
                    vault.last_accessed as i64,
                    vault.img,
                ],
            ).map_err(|e| anyhow!("DB insert failed: {}", e))?;

            let vault_id = conn.last_insert_rowid();

            let md: Vec<u8> = vec![10, 20, 30, 40]; // ! change md later 

            let cipher = crypto::encrypt(md.as_ref(), components, Self::KEY_ITR)?;

            conn.execute(
                "INSERT INTO vaultsMd (vault_id,data,nonce,salt) VALUES ( ?, ?, ?, ?)",
                params![vault_id, cipher[0], cipher[1], cipher[2]],
            )
            .map_err(|e| anyhow!("DB insert failed: {}", e))?;

            println!("Vault Created Succesfuly with id : {}", vault_id);

            Ok::<i64>(vault_id)
        });

        let res = handle
            .join()
            .map_err(|e| anyhow!("Thread panicked: {:?}", e))??; // double ?: join error + thread result

        Ok(res)
    }

    pub fn add_conn(&self, provider: String, data: Vec<u8>, vault_id: i64) -> Result<i64> {
        // let vault_key = if let Some(vk) = self.vault_keys.get_mut(vault_id) {};
        let conn1 = self.conn.clone();
        let vk1 = self.vault_keys.clone();
        let mk = self.master_key.clone();

        let handle = thread::spawn(move || {
            let conn = conn1.lock().unwrap();
            let vault_keys = vk1.lock().unwrap();
            let mut components: Vec<&str> = Vec::new();
            let master_key = mk.as_str();
            components.push(master_key);

            if let Some(vk) = vault_keys.get(&vault_id) {
                components.push(vk.as_str());
            };
            let cipher = crypto::encrypt(data.as_ref(), components, Self::KEY_ITR)?;

            // let card = Card::new(&data, components, database::key_itr, provider, vault_id);

            conn.execute(
                "INSERT INTO connections (vault_id, provider, data, nonce,salt) VALUES (?, ?, ?, ?, ?)",
                params![
                  vault_id,
                  provider,
                  cipher[0],
                  cipher[1],
                  cipher[2],
                ],
            ).map_err(|e| anyhow!("DB insert failed: {}", e))?;

            let id = conn.last_insert_rowid();

            println!("Connection Addded Succesfullhy with id : {}", id);

            Ok::<i64>(id)
        });

        let res = handle
            .join()
            .map_err(|e| anyhow!("Thread panicked: {:?}", e))??;

        Ok(res)
    }

    pub fn open_vault(&self, vault_id: i64, password: String) -> Result<VaultMd> {
        let conn1 = self.conn.clone();
        let vk1 = self.vault_keys.clone();
        let mk = self.master_key.clone();

        let handle = thread::spawn(move || {
            let conn = conn1.lock().unwrap();
            let mut vault_keys = vk1.lock().unwrap();

            let components: Vec<&str> = vec![mk.as_str(), password.as_str()];

            let mut stmt = conn
                .prepare("SELECT * FROM vaultsMd where id = ?1")
                .map_err(|e| anyhow!("DB prepare failed: {}", e))?;

            let res = stmt
                .query_row(params![vault_id], |row| {
                    // NOTE : here anyhow errors not work
                    let id = row.get(0)?;
                    let vault_id = row.get(1)?;
                    let data: Vec<u8> = row.get(2)?;
                    let nonce = row.get(3)?;
                    let salt = row.get(4)?;

                    let plain =
                        crypto::decrypt(components, Self::KEY_ITR, salt, nonce, data.as_ref())
                            .map_err(|e| rusqlite::Error::InvalidQuery)?;

                    if let None = vault_keys.get(&vault_id) {
                        vault_keys.insert(vault_id, password.clone());
                    }

                    rusqlite::Result::Ok(VaultMd {
                        data: plain,
                        id,
                        vault_id,
                    })
                })
                .map_err(|e| anyhow!("DB query failed: {}", e))?;

            Ok::<VaultMd>(res)
        });

        let res = handle
            .join()
            .map_err(|e| anyhow!("Thread panicked: {:?}", e))??;

        Ok(res)
    }

    /// get specific card with card_id and vault_id , vault must be opened
    pub fn get_card(&self, id: i64, vault_id: i64) -> Result<Card> {
        let conn1 = self.conn.clone();
        let vk1 = self.vault_keys.clone();
        let mk = self.master_key.clone();

        let handle = thread::spawn(move || {
            let conn = conn1.lock().unwrap();
            let vault_keys = vk1.lock().unwrap();

            let mut components: Vec<&str> = vec![mk.as_str()];

            if let Some(pass) = vault_keys.get(&vault_id) {
                components.push(pass.as_str());
            } else {
                bail!(
                    "Password to decrypt Card {} doesnot exists on vault {}",
                    id,
                    vault_id
                );
            }

            let mut stmt = conn
                .prepare("SELECT * FROM connections where id = ?1")
                .map_err(|e| anyhow!("DB prepare failed: {}", e))?;

            let res = stmt
                .query_row(params![vault_id], |row| {
                    // NOTE : here anyhow errors not work
                    let id = row.get(0)?;
                    let vault_id = row.get(1)?;
                    let provider = row.get(2)?;

                    let data: Vec<u8> = row.get(3)?;
                    let nonce = row.get(4)?;
                    let salt = row.get(5)?;

                    let plain =
                        crypto::decrypt(components, Self::KEY_ITR, salt, nonce, data.as_ref())
                            .map_err(|e| rusqlite::Error::InvalidQuery)?;

                    rusqlite::Result::Ok(Card {
                        data: Some(plain),
                        provider,
                        id,
                        vault_id,
                    })
                })
                .map_err(|e| anyhow!("DB query failed: {}", e))?;

            Ok::<Card>(res)
        });

        let res = handle
            .join()
            .map_err(|e| anyhow!("Thread panicked: {:?}", e))??;

        Ok(res)
    }

    /// get bunch of vaults details
    pub fn get_vaults(&self, _limit: Option<i64>) -> Result<Vec<Vault>> {
        let conn1 = self.conn.clone();
        let at_vf = self.vaults_fetched.clone();
        let vf_val = at_vf.load(Ordering::SeqCst);
        let limit = _limit.unwrap_or_else(|| 10);

        let handle = thread::spawn(move || {
            let conn = conn1.lock().unwrap();
            // let vaults_fetched = vf.

            let mut stmt = conn
                .prepare("SELECT * FROM vaults ORDER BY created_at DESC LIMIT ?1 OFFSET ?2")
                .map_err(|e| anyhow!("DB prepare failed: {}", e))?;

            let res_itr = stmt.query_map(params![limit, vf_val], |row| {
                at_vf.fetch_add(1, Ordering::SeqCst); // increment total fetched 

                rusqlite::Result::Ok(Vault {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    desc: Some(row.get(2)?),
                    created_at: row.get(3)?,
                    last_accessed: row.get(4)?,
                    img: row.get(5)?,
                })
            })?;

            // res.
            // let card: Vec<Card> = res_itr.collect::<SqlResult<Vec<_>>>()?;

            let vaults: Vec<Vault> = res_itr.collect::<SqlResult<Vec<_>>>()?;

            Ok::<Vec<Vault>>(vaults)
        });
        let res = handle
            .join()
            .map_err(|e| anyhow!("Thread panicked: {:?}", e))??;

        Ok(res)
    }

    /// for now no paginations for cards -> u get bunch of card details
    pub fn get_cards(&self, vault_id: i64) -> Result<Vec<Card>> {
        if let None = self.vault_keys.lock().unwrap().get(&vault_id) {
            println!("Vault is not opened  cannot fetch its cards");
            bail!("Vault is not opened cannot fetch cards");
        }

        let conn1 = self.conn.clone();

        let handle = thread::spawn(move || {
            let conn = conn1.lock().unwrap();
            // let vaults_fetched = vf.

            let mut stmt = conn
                .prepare("SELECT * FROM connections WHERE vault_id = ?1")
                .map_err(|e| anyhow!("DB prepare failed: {}", e))?;

            let res_itr = stmt.query_map([vault_id], |row| {
                rusqlite::Result::Ok(Card {
                    id: row.get(0)?,
                    vault_id: row.get(1)?,
                    provider: row.get(2)?,
                    data: None,
                    // vault_id: row.get(2)?,
                })
            })?;

            let cards: Vec<Card> = res_itr.collect::<SqlResult<Vec<_>>>()?;

            Ok::<Vec<Card>>(cards)
        });
        let res = handle
            .join()
            .map_err(|e| anyhow!("Thread panicked: {:?}", e))??;

        Ok(res)
    }
}
