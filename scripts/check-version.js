import { readFileSync } from 'node:fs'

const pkg = JSON.parse(readFileSync(new URL('../package.json', import.meta.url), 'utf8'))
const tauri = JSON.parse(
  readFileSync(new URL('../src-tauri/tauri.conf.json', import.meta.url), 'utf8'),
)
const cargo = readFileSync(new URL('../src-tauri/Cargo.toml', import.meta.url), 'utf8')
const cargoVer = cargo.match(/^version\s*=\s*"([^"]+)"/m)?.[1]

let ok = true
if (pkg.version !== tauri.version) {
  console.error(`Version mismatch: package.json ${pkg.version} != tauri.conf.json ${tauri.version}`)
  ok = false
}
if (cargoVer && pkg.version !== cargoVer) {
  console.error(`Version mismatch: package.json ${pkg.version} != Cargo.toml ${cargoVer}`)
  ok = false
}
if (ok) console.log(`Version sync OK: ${pkg.version}`)
process.exit(ok ? 0 : 1)
