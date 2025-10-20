export interface VaultMd {
  id: number;
  vault_id: number;
  data: Uint8Array | Blob | ArrayBuffer;
}

export type CardSchema = {
  ["App"]: {
    id?: number; // uuid
    vaultid: number;
    provider: string;
    username?: string;
    email?: string;
    desc?: string;
    created_at: number;
  };
  ["Form"]: {
    provider: string;
    username?: string;
    email?: string;
    desc?: string;
    password: string;
  };
};

export type VaultSchema = {
  id?: number; //uuid
  name: string;
  desc?: string;
  created_at: number;
  last_accesed: number;
  img?: Uint8Array | Blob | ArrayBuffer;
};

// export interface VaultSchema {}

export interface FormProps {
  name: string;
  type?: "pass" | "txt" | "cnfm-pass";
  placeholder?: string;
  required?: boolean;
  stMeter?: boolean;
}
