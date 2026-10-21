# Changelog

All notable changes to MH-AIO. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
versioning follows [SemVer](https://semver.org/spec/v2.0.0.html).

Row counts quoted here come from [`docs/data-coverage.md`](docs/data-coverage.md), a generated
per-table × per-game matrix.

## [Unreleased]

Nothing yet.

## [0.2.0] — 2026-10-20

First release since 0.1.0. **0.1.1 was never published** — it was version-bumped in the repo but
never tagged, so this release covers everything from 0.1.0 forward.

### Added

**Data / specs**

- **MHW + Iceborne Endemic Life** (spec `015`) — 33 curated creatures (8 bugs, 23 small-monster
  packs, 2 special) in a new `endemic_life` table, sharing the Wilds browser at `/mhw/endemic`.
  `kind` uses the Slinger-ammo yield taxonomy for parity with Wilds.
- **MH Wilds quests, charms, small monsters + endemic** (specs `012`–`014`) — 174 quests
  (Optional 81 / Event 60 / Assignment 33), 76 talismans, 34 small monsters, 70 endemic creatures.
- **MH Rise + Sunbreak to 100%** (spec `011`) — 1,120 status-ailment rows, monster part-break
  data, 270 trap rows, buddy gear (322 Palico weapons / 362 Palamute armor), rampage (247 skills,
  6,822 weapon links, 26 decorations), 11,849 talisman generator rows, 178 combines.
- **MHW + Iceborne to 100%** (spec `010`) — 721 ailment rows, 465 trap rows, 317 hunter charms
  with a rank-tree browser, +58 set-bonus `skill2` fills, descriptions for all 3,544 weapons /
  1,595 armor / 94 monsters, deco acquisition metadata 398/404, Melder / shop / gather sources.
- **MHP3rd completeness** (spec `001`) — 50,635 excavated charms with skill search, Comrades
  gear (92/154/52), 180 trap tools, 398 combines (276 Normal + 122 Alchemy).
- **MH2G gap completion** — drops, weaknesses, items, quests and gear expansion against the
  retail-UMD extract.
- **`docs/data-coverage.md`** — generated per-table × per-game row-count matrix, now the single
  source of truth for every count quoted anywhere in the docs.

**UI**

- New `charms/` browser — MHP3rd excavated charms, MHW hunter charms, MHR talismans, Wilds talismans.
- New `comrades/` browser — MHP3rd + MHR buddy gear.
- New `tools/` section — MHW Mantles, Boosters and Palico Gadgets.
- New `endemic/` browser — MHW + MH Wilds.
- Offline icon sets for items, monsters, weapons, armor, quests, skills, decorations and builds.
- Rise/Wilds portrait (`-sm`) and master (`icon_url_lg`) thumbnail variants.
- Sidebar hover-warm query prefetching.

**Infrastructure**

- Release pipeline now builds **five targets**: Windows x64, macOS arm64, macOS x64, Linux x64,
  Linux arm64 (up from three — Intel Macs and arm64 Linux gained coverage).
- GitHub release notes generated from `CHANGELOG.md` plus GitHub's auto-generated commit notes.

### Fixed

- **Foreign-key panic on warm boot** — `patch_mhp3rd_gear` deleted skills `10101`/`10102` before
  deleting the `#engname` decoration that references them, so SQLite FK enforcement aborted the
  seed with an opaque `FOREIGN KEY constraint failed`. Decorations are now deleted first, every
  patch in `apply_data_patches()` logs its own name on failure, and
  `mhp3rd_gear_fix_fk_regression_1931524` locks the behaviour in.
- **MHW small-monster size drift** — `seed_mhw_monsters` uses `INSERT OR IGNORE`, which never
  corrects an existing column, so 13 of 23 small monsters (Jagras, Mernos, Vespoid, Kelbi,
  Shamos, Girros, Barnos, Gajau, Gastodon, Hornetaur, Kestodon, Noios, Raphinos) were stuck at
  `size='Large'` and vanished from the monsters "Small" filter. `patch_mhw_015_small_size` now
  backfills `size` from the committed JSON.
- **`npm audit` failing CI** — four transitive advisories (`brace-expansion`, `devalue`,
  `postcss-selector-parser`, `source-map-js`) resolved via pinned `overrides`. Now 0 vulnerabilities.
- **Apple Silicon "damaged app" error** — macOS bundles are ad-hoc signed
  (`bundle.macOS.signingIdentity = "-"`), so Gatekeeper no longer reports downloaded releases as
  damaged. 0.1.0 shipped unsigned, which broke this for every M-series user.

### Changed

- Documentation refreshed against the live database — roughly 60 stale counts corrected across
  `README.md`, `STATUS.md`, `roadmap.md`, `AGENTS.md`, `CONTRIBUTING.md`,
  `docs/fidelity-report.md` and `spec/constitution/*`.
- `spec/constitution/*` rewritten: the per-game status table was wrong for 4 of 5 titles and the
  SDD index stopped at spec `009`.
- Historical specs keep their original numbers (retro-anchor convention) and gained additive
  `> **Superseded counts**` banners pointing at the corrected values.
- CI Node pinned to `lts/*`; Rust build cache unchanged.

### Known limitations

- **macOS**: ad-hoc signed, **not notarized**. Notarization requires a paid Apple Developer
  account, and only the Account Holder can create a Developer ID Application certificate. Users
  must allow the app once via System Settings → Privacy & Security. Intel builds are ad-hoc
  signed as well; Rosetta 2 is not required by MH-AIO, but the x64 binary runs natively on Intel.
- **Windows**: installers are **unsigned**, so SmartScreen will warn until the file accrues
  reputation. Note that since Microsoft's 2024 policy change even EV certificates no longer bypass
  the warning on a first release — signing mainly buys long-term reputation. Wire it up via
  `bundle.windows` (`certificateThumbprint` / `digestAlgorithm` / `timestampUrl`) plus the
  `WINDOWS_CERTIFICATE` and `WINDOWS_CERTIFICATE_PASSWORD` secrets.
- **No auto-updater**: the Tauri updater plugin is not configured, so no `latest.json` manifest is
  published. Update by downloading a new release.
- **MH Wilds** has no `quest_rewards`: the upstream wiki exposes reward _text_, not per-item
  counts or probabilities.
- MHW/Iceborne endemic life is a curated cross-category view, not a game mechanic — World has no
  in-game Endemic Life system (Wilds does).

## [0.1.0] — 2026-08-29

- Initial public release: MH2G (Freedom Unite) as a verified-complete MVP against the retail UMD,
  plus MHW+Iceborne and MHP3rd baselines.
- Armor Set Search (Athena's A.S.S. Rust port), per-game theming, global search, per-rarity
  weapon icons, Chest/Smith ordering.
- Ships 7 artifacts: Windows `.exe` + `.msi`, macOS arm64 `.dmg` + `.app.tar.gz`, Linux `.deb`,
  `.rpm`, `.AppImage`.

[Unreleased]: https://github.com/Seb-fd/mh-aio/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Seb-fd/mh-aio/compare/0.1.0...v0.2.0
[0.1.0]: https://github.com/Seb-fd/mh-aio/releases/tag/0.1.0
