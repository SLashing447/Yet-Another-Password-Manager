// import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { encode, decode } from "@msgpack/msgpack";

import type { CardSchema, VaultMd, VaultSchema } from "./types";
import { invoke } from "@tauri-apps/api/core";
import { appDataDir } from "@tauri-apps/api/path";
// import { resolve } from "@tauri-apps/api/path";

export enum DataType {
  Card = "card",
  Cards = "cards",
  Db = "db",
  Vault = "vault",
}

type Payload =
  | { type: DataType.Card; data: CardSchema["App"] }
  | { type: DataType.Vault; data: VaultSchema; password: String };

type FetchType =
  | { type: DataType.Cards; vault_id: number }
  | { type: DataType.Vault; limit?: number }
  | { type: DataType.Card; vault_id: number; card_id: number };

type winFn = "Close" | "Min" | "Max" | "UnMax";

type UnlockType =
  | {
      type: DataType.Db;
      password: String;
    }
  | {
      type: DataType.Vault;
      vault_id: number;
      password: String;
    };

class Database {
  private appWindow;
  private AppDataPath: String | null = null;

  constructor() {
    // const appWindow = getCurrentWindow();
    this.appWindow = getCurrentWindow();
    appDataDir()
      .then((path) => (this.AppDataPath = path))
      .catch((e) => {
        console.error("APp Data dir fetch fail");
      });
  }

  async isNewProfile(): Promise<boolean> {
    return new Promise(async (resolve, reject) => {
      try {
        if (!this.AppDataPath) reject("Fail : AppDataPath is not defined");
        //
        const isNewProfile: boolean = await invoke("file_exists", {
          frontend_path: this.AppDataPath,
        });
        resolve(isNewProfile);
      } catch (e) {
        reject(`Fail : isNewProfile failed with ${e}`);
      }
    });
  }

  // unlock vault or entire database
  async unlock(ut: UnlockType) {
    // console.log(`Unlocking ${id} with ${password}`);
    switch (ut.type) {
      case DataType.Db:
        return this.open_db(ut.password);
      case DataType.Vault:
        return this.unlock_vault(ut.vault_id, ut.password);
    }
  }

  /**
   * Unlocks specific vault with password
   */
  private async unlock_vault(
    vault_id: number,
    password: String
  ): Promise<VaultMd> {
    return new Promise(async (resolve, reject) => {
      try {
        const vault_md: VaultMd = await invoke("open_vault", {
          vault_id,
          password,
        });
        resolve(vault_md);
      } catch (e) {
        reject(`Fail: unlock vault ${vault_id} failed with : ${e}`);
      }
    });
  }

  /**
   * opens db if not exists with password `password`
   */
  private async open_db(password: String): Promise<String> {
    return new Promise(async (resolve, reject) => {
      try {
        await invoke("init_db", {
          password,
        });
        resolve("Open Db sucess");
      } catch (e) {
        reject(`Fail: Db Opening failed with  : ${e}`);
      }
    });
  }

  async get(fetch: FetchType) {
    switch (fetch.type) {
      case DataType.Cards: {
        return this.get_cards(fetch.vault_id);
      }
      case DataType.Card: {
        return this.get_card(fetch.vault_id, fetch.card_id);
      }
      case DataType.Vault: {
        return this.get_vaults(fetch.limit);
      }
    }
  }

  private async get_card(
    vault_id: number,
    card_id: number
  ): Promise<CardSchema["App"]> {
    return new Promise(async (resolve, reject) => {
      try {
        const card: CardSchema["App"] = await invoke("get_card", {
          card_id,
          vault_id,
        });
        resolve(card);
      } catch (e) {
        reject(`Fail : fetch card fail with : ${e}`);
      }
    });
  }
  private async get_cards(vault_id: number): Promise<CardSchema["App"][]> {
    return new Promise(async (resolve, reject) => {
      try {
        const cards: CardSchema["App"][] = await invoke("get_cards", {
          vault_id,
        });
        resolve(cards);
      } catch (e) {
        reject(`Fail : fetch cards fail with : ${e}`);
      }
    });
  }
  private async get_vaults(limit?: number): Promise<VaultSchema[]> {
    return new Promise(async (resolve, reject) => {
      try {
        const vaults: VaultSchema[] = await invoke("get_vaults", {
          limit,
        });
        resolve(vaults);
      } catch (e) {
        reject(`Fail : fetch vaults fail with : ${e}`);
      }
    });
  }
  async post(payload: Payload) {
    switch (payload.type) {
      case DataType.Card: {
        return this.insert_card(payload.data);
      }
      case DataType.Vault: {
        return this.insert_vault(payload.data, payload.password);
      }
    }
  }
  private async insert_card(data: CardSchema["App"]): Promise<String> {
    return new Promise(async (resolve, reject) => {
      try {
        // conver the senstive fields to binary
        const bytes = encode({
          username: data.username,
          desc: data.desc,
          created_at: data.created_at,
          email: data.email,
        });

        await invoke("insert_card", {
          provider: data.provider,
          data: bytes,
          vault_id: data.vaultid,
        });
        resolve("OK : Insert Card");
      } catch (e) {
        reject(`Fail : Insert Card Failed with : ${e}`);
      }
    });
  }
  private async insert_vault(
    data: VaultSchema,
    password: String
  ): Promise<String> {
    return new Promise(async (resolve, reject) => {
      try {
        await invoke("insert_card", {
          vault: data,
          password,
        });
        resolve("OK : Insert Vault");
      } catch (e) {
        reject(`Fail : Insert Vault Failed with : ${e}`);
      }
    });
  }

  async windowFn(type: winFn) {
    switch (type) {
      case "Close":
        await this.appWindow.close();
        break;
      case "Max":
        await this.appWindow.maximize();
        break;
      case "Min":
        await this.appWindow.minimize();
        break;
      case "UnMax":
        await this.appWindow.unmaximize();
        break;
      default:
        console.log("Wrong command for window Functions");
        break;
    }
  }
}

const Operator = new Database();
export default Operator;
