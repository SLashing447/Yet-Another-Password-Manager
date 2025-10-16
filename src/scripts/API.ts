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

  async addVault(data: VaultSchema["on_create"]) {
    await invoke("add_vault", {
      data: data,
    });
  }

  async addCard(data: CardSchema["on_create"]) {
    await invoke("add_vault", {
      data: data,
    });
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
