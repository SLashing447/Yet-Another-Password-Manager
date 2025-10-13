import type { Warning } from "svelte/compiler";
import { writable, type Writable } from "svelte/store";

export const routes: Writable<string[]> = writable(["Connections"]);
export function routeBack() {
  routes.update((arr) => {
    if (arr.length === 1) return arr;
    arr.pop();
    return arr;
  });
}

export function routeTo(name: string) {
  routes.update((arr) => {
    arr.push(name);
    return arr;
  });
}
export const selected_vault: Writable<string> = writable("0");
