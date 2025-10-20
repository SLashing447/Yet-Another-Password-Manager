import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import msgpack from "@msgpack/msgpack";

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
  private appWindow;

  constructor() {
    // const appWindow = getCurrentWindow();
    this.appWindow = getCurrentWindow();
  }

  async addVault(data: VaultSchema["on_create"]) {
    const binary: Uint8Array = msgpack.encode({
      ...(data.desc && { desc: data.desc }),
    });

    await invoke("add_vault", {
      name: data.name,
      bin: binary,
      password: data.password,
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

  async closeWindow() {
    await this.appWindow.close();
  }
  async minimizeWindow() {
    await this.appWindow.minimize();
  }
  async maximizeWindow() {
    await this.appWindow.maximize();
  }
  async unmaximizeWindow() {
    await this.appWindow.unmaximize();
  }
}

const Operator = new Database();
export default Operator;
