export interface CardSchema {
  provider: string;
  username?: string;
  email?: string;
  desc?: string;
  password: string;
}
export interface VaultSchema {
  name: string;
  desc?: string;
  last_accesed: number;
  created_at: number;
}

export interface FormProps {
  name: string;
  type?: "pass" | "txt" | "cnfm-pass";
  placeholder?: string;
  required?: boolean;
  stMeter?: boolean;
}
