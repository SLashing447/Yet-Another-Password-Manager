/**
 * Returns password strength score (0-5) and label, prioritizing key rules
 * @param {string} password
 * @returns {{ score: number, label: string }}
 */
export function checkPasswordStrength(password: string): {
  score: number;
  label: string;
  color: string;
} {
  let score = 0;

  // --- Highest priority rules ---
  // Rule 1: minimum length >= 8 (must-have)
  if (password.length >= 8) score += 2; // double weight
  else return { score: 0, label: "Very Weak", color: "red" }; // fail fast if too short

  // --- Secondary rules (1 point each) ---
  if (/[a-z]/.test(password) && /[A-Z]/.test(password)) score++; // mixed case
  if (/\d/.test(password)) score++; // number
  if (/[\W_]/.test(password)) score++; // special char
  if (password.length >= 12) score++; // extra long

  // Cap max score to 5 (6 levels: 0-5)
  if (score > 5) score = 5;

  const labels = ["Very Weak", "Weak", "Fair", "Good", "Strong", "Very Strong"];
  const colors = [
    "#f44336", // red
    "#ff7043", // orange-red
    "#ffb74d", // amber
    "#ffeb3b", // yellow
    "#aed581", // light green
    "#4caf50", // green
  ];
  return { score, label: labels[score], color: colors[score] };
}
