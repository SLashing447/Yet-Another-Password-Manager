// export interface CardSchema {
//   id: string; // uuid
//   provider: string;
//   username?: string;
//   email?: string;
//   desc?: string;
//   password: string;
// }

export type CardSchema = {
  ["on_create"]: {
    provider: string;
    desc?: string;
    email?: string;
    password: string;
  };
  ["App"]: {
    id: string; // uuid
    provider: string;
    username?: string;
    email?: string;
    desc?: string;
    created_at: number;
  };
};

export type VaultSchema = {
  ["on_create"]: {
    name: string;
    desc?: string;
    password?: string;
  };
  ["App"]: {
    id?: string; //uuid
    name: string;
    desc?: string;
    last_accesed?: number;
    created_at?: number;
  };
};

// export interface VaultSchema {}

export interface FormProps {
  name: string;
  type?: "pass" | "txt" | "cnfm-pass";
  placeholder?: string;
  required?: boolean;
  stMeter?: boolean;
}
