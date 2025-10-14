import { get, writable, type Writable } from "svelte/store";

export const selected_vault: Writable<string> = writable("0");
export const path: Writable<string[]> = writable(["connections"]);

export function routeTo(name: string) {
  if (name === "-1" && get(path).length > 1) {
    path.update((arr) => {
      arr.pop();
      return arr;
    });
  } else {
    path.update((arr) => {
      arr.push(name);
      return arr;
    });
  }
}
