## Yes
## Yet Another Password Manager
## You Need 😂 , all the features u expect

---

### this is how the backend works lol
The backend is rust tauri so its pretty simple
and it uses  sql-lite db
```rust
User passphrase -> Master Key
Vault Key -> derived from (Master Key + optional Vault Password)
Vault Name / Record Provider -> plaintext or indexed for search
Vault data: Vec<u8> -> encrypt with Vault Key
Record data: Vec<u8> -> encrypt with Vault Key
Search -> hash/index only (no sensitive info exposed)
```
