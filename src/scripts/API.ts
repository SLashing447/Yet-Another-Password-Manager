import type { CardSchema, VaultSchema } from "./types";

export enum DataType {
  card = "card",
  vault = "vault",
}
type DataSchema = {
  [DataType.card]: CardSchema;
  [DataType.vault]: VaultSchema;
};

class Database {
  constructor() {}

  async post<T extends DataType>(type: T, data: DataSchema[T]) {
    console.log("Posting : ", type, " with data : \n ", data);
  }
  async unlock(type: "db" | "vault", password: string) {}
  async delete<T extends DataType>(type: T, password: string) {}
}

const Operator = new Database();
export default Operator;
