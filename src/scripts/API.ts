import { invoke } from "@tauri-apps/api/core";
import type { CardSchema, VaultSchema } from "./types";

export enum DataType {
  Card = "card",
  Vault = "vault",
}
type DataSchema = {
  [DataType.Card]: CardSchema;
  [DataType.Vault]: VaultSchema;
};

class Database {
  constructor() {}

  async post<T extends DataType>(type: T, data: DataSchema[T]) {
    try {
      const result = await invoke("post", {
        data_type: "Card",
      });
      console.log("Backend replied:", result);
    } catch (err) {
      console.error("Error calling backend:", err);
    }
  }
  async unlock(type: "db" | "vault", password: string, id?: string) {
    console.log(`Unlocking ${type} with ${password}`);
  }
  async delete<T extends DataType>(type: T, password: string, id: string) {}
  async fetch<T extends DataType>(type: T, limit?: number) {}

  // this is cached
  async fetchCardDetails(id: string, vault_id: string) {}
}

const Operator = new Database();
export default Operator;
