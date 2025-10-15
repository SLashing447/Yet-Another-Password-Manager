import { get, writable, type Writable } from "svelte/store";

export const selected_vault: Writable<string> = writable("0");
export const path: Writable<string[]> = writable(["connections"]);


async function fetchLogo(query: string) {
  const res = await fetch(
    `https://autocomplete.clearbit.com/v1/companies/suggest?query=${query}`,
  );
  const data = await res.json();
  return data[0]?.logo ?? `https://cdn.simpleicons.org/${query.toLowerCase()}`;
}

export function routeTo(name: string, override?: boolean) {
  let p = get(path);
  if (p.length === 1 && name === "-1") return;
  if (p.includes(name)) return;
  if (name === "-1" && p.length > 1) {
    path.update((arr) => {
      arr.pop();
      return arr;
    });
  } else {
    if (override) {
      path.set([name]);
      return;
    }
    path.update((arr) => {
      arr.push(name);
      return arr;
    });
  }
}
