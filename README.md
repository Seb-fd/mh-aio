# MH-AIO (Monster Hunter All-In-One Encyclopedia)

A comprehensive, cross-platform desktop encyclopedia and toolkit for Monster Hunter games, built with **Tauri v2**, **Rust**, **Svelte 5**, and **SQLite**.

---

## Tech Stack

- **Frontend:** Svelte 5, Vite, TypeScript, Tailwind CSS v4, shadcn-svelte (bits-ui)
- **Backend:** Rust (Tauri v2 core)
- **Database:** SQLite (`rusqlite` with bundled feature, WAL mode)
- **Routing:** SvelteKit (Client-side SPA mode with static adapter and `fallback: 'index.html'`)

---

## Supported Games

The app supports multiple titles through dynamic game routing (`[game]`):
- `mhw` — Monster Hunter: World
- `mhr` — Monster Hunter Rise
- `mhwilds` — Monster Hunter Wilds
- `mhp3rd` — Monster Hunter Portable 3rd
- `mh2g` — Monster Hunter Freedom Unite / Dos (MH2G)

Each game section covers:
- Monsters
- Weapons
- Armor
- Quests
- Items
- Skills
- Builds

---

## Getting Started

### Prerequisites

Ensure you have the following installed on your system:
- **Node.js** (LTS recommended) & npm / pnpm / bun
- **Rust** (stable toolchain via [rustup](https://rustup.rs/))
- **Tauri v2 Prerequisites** (depends on your OS: WebView2 on Windows, WebKitGTK on Linux, Xcode command line tools on macOS)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Seb-fd/mh-aio.git
   cd mh-aio
   ```

2. Install frontend dependencies:
   ```bash
   npm install
   ```

---

## Development Commands

```bash
# Run frontend only (Vite dev server on port 1420)
npm run dev

# Run full desktop app (Svelte frontend + Rust backend via Tauri)
npx tauri dev

# Build production desktop application
npx tauri build

# Build Rust backend only
cargo build --manifest-path src-tauri/Cargo.toml
```

---

## Project Structure

- `src/` — Svelte 5 frontend source code, components, stores, and routes.
  - `src/routes/[game]/` — Dynamic game encyclopedia sub-routes.
  - `src/lib/components/ui/` — Reusable UI primitives (shadcn-svelte).
- `src-tauri/` — Rust backend source code, database schema/queries, and Tauri configuration.
  - `src-tauri/src/db/` — SQLite schema definitions and query handlers.
  - `src-tauri/src/commands/` — Tauri commands registered in `lib.rs`.

---

## License

This project is licensed under the MIT License.
