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
  lastAccesed: number;
  createdAt: number;
}
