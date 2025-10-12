export async function generateTOTP(
  secretBase32: string,
  digits: number = 6,
  period: number = 30,
  algorithm: string = "SHA-1"
): Promise<string> {
  // Validation code...
  if (period < 1) throw new Error("Period must be at least 1 second");
  if (digits < 1 || digits > 10)
    throw new Error("Digits must be between 1 and 10");

  const validAlgorithms = ["SHA-1", "SHA-256", "SHA-384", "SHA-512"];
  if (!validAlgorithms.includes(algorithm.toUpperCase())) {
    throw new Error(`Algorithm must be one of: ${validAlgorithms.join(", ")}`);
  }

  try {
    // Decode Base32 secret
    const decodedSecret = base32Decode(secretBase32);

    // Calculate time counter
    const timeStamp = Math.floor(Date.now() / 1000);
    const timeCounter = Math.floor(timeStamp / period);

    // Convert time counter to 8-byte buffer
    const timeBuffer = new ArrayBuffer(8);
    const timeView = new DataView(timeBuffer);
    const high = Math.floor(timeCounter / 0x100000000);
    const low = timeCounter % 0x100000000;
    timeView.setUint32(0, high, false);
    timeView.setUint32(4, low, false);

    // **FIX: Create proper ArrayBuffer for the key**
    const keyBuffer = new ArrayBuffer(decodedSecret.length);
    new Uint8Array(keyBuffer).set(decodedSecret);

    // Create HMAC key - no more type error!
    const key = await crypto.subtle.importKey(
      "raw",
      keyBuffer, // Pass ArrayBuffer directly
      {
        name: "HMAC",
        hash: algorithm.toUpperCase(),
      },
      false,
      ["sign"]
    );

    // Generate HMAC and extract OTP
    const signature = await crypto.subtle.sign("HMAC", key, timeBuffer);
    const hmac = new Uint8Array(signature);

    const offset = hmac[hmac.length - 1] & 0x0f;
    const binaryCode =
      ((hmac[offset] & 0x7f) << 24) |
      ((hmac[offset + 1] & 0xff) << 16) |
      ((hmac[offset + 2] & 0xff) << 8) |
      (hmac[offset + 3] & 0xff);

    return (binaryCode % Math.pow(10, digits)).toString().padStart(digits, "0");
  } catch (error) {
    throw new Error(
      `TOTP generation failed: ${
        error instanceof Error ? error.message : "Unknown error"
      }`
    );
  }
}

function base32Decode(base32: string): Uint8Array {
  const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
  const cleanInput = base32.replace(/=+$/, "").toUpperCase();

  let bits = "";
  for (const char of cleanInput) {
    const value = alphabet.indexOf(char);
    if (value === -1) {
      throw new Error(`Invalid Base32 character: ${char}`);
    }
    bits += value.toString(2).padStart(5, "0");
  }

  const bytes: number[] = [];
  for (let i = 0; i < bits.length; i += 8) {
    if (bits.length - i >= 8) {
      bytes.push(parseInt(bits.substring(i, i + 8), 2));
    }
  }

  return new Uint8Array(bytes);
}
