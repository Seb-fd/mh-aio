// Shared accent-insensitive + case-insensitive key, mirroring the Rust
// `norm_key` in src-tauri/src/db/queries.rs so list filters and the backend
// global search behave consistently.
const FOLD: Record<string, string> = {
  à: 'a', á: 'a', â: 'a', ã: 'a', ä: 'a', å: 'a',
  ç: 'c',
  è: 'e', é: 'e', ê: 'e', ë: 'e',
  ì: 'i', í: 'i', î: 'i', ï: 'i',
  ñ: 'n',
  ò: 'o', ó: 'o', ô: 'o', õ: 'o', ö: 'o',
  ù: 'u', ú: 'u', û: 'u', ü: 'u',
  ý: 'y', ÿ: 'y',
};

export function normKey(s: string): string {
  return s
    .toLowerCase()
    .split('')
    .map((ch) => FOLD[ch] ?? ch)
    .join('');
}
