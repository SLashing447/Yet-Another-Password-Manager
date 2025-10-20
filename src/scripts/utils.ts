import { get, writable, type Writable } from "svelte/store";

export const selected_vault: Writable<number> = writable(0);
export const isAuthenticated: Writable<boolean> = writable(true);

export const path: Writable<string[]> = writable(["connections"]);

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

export function getUnixTime(): number {
  const unixTimeSeconds = Math.floor(Date.now() / 1000);
  return unixTimeSeconds;
}

export type PasswordStrength = {
  score: number; // 0–10
  color: string;
  verdict: string;
};

export function evaluatePassword(password: string): PasswordStrength {
  if (!password) return { score: 0, color: "transparent", verdict: "Empty" };

  let score = 0;

  // --- 1. Length scoring ---
  const lengthScore = Math.min(password.length / 2, 5); // up to 5 points
  score += lengthScore;

  // --- 2. Character diversity ---
  const hasLower = /[a-z]/.test(password);
  const hasUpper = /[A-Z]/.test(password);
  const hasNumber = /\d/.test(password);
  const hasSymbol = /[^A-Za-z0-9]/.test(password);
  const diversity = [hasLower, hasUpper, hasNumber, hasSymbol].filter(
    Boolean
  ).length;
  score += diversity * 1.5; // up to 6 points total combined w/ length

  // --- 3. Repetition & pattern penalty ---
  if (/(\w)\1{2,}/.test(password)) score -= 1; // repeating chars
  if (/123|abc|qwerty|password|letmein|1111/i.test(password)) score -= 2; // common patterns

  // --- 4. Dictionary weakness ---
  const weakWords = ["password", "admin", "welcome", "test", "default"];
  if (weakWords.some((w) => password.toLowerCase().includes(w))) score -= 2;

  // --- 5. Entropy-like scaling ---
  const uniqueChars = new Set(password).size;
  if (uniqueChars < 4) score -= 2;
  else if (uniqueChars > 8) score += 1;

  // --- Clamp & normalize ---
  score = Math.max(0, Math.min(10, Math.round(score)));

  // --- Color & verdict ---
  let color = "red";
  let verdict = "Very Weak";
  if (score >= 8) {
    color = "teal";
    verdict = "Excellent";
  } else if (score >= 6) {
    color = "green";
    verdict = "Strong";
  } else if (score >= 4) {
    color = "yellow";
    verdict = "Moderate";
  } else if (score >= 2) {
    color = "orange";
    verdict = "Weak";
  }

  return { score, color, verdict };
}
