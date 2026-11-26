# KatWork fork of SoundCraft

This repository is the KatWork fork of SoundCraft (https://github.com/storytold/soundcraft). KatWork builds the app's CLI and web
build from this fork. It carries the patches listed below; everything else is upstream code.

Based on SoundCraft by the ArtCraft team (https://github.com/storytold/soundcraft). This build is not an ArtCraft product.

## Fork base

- Fork base (this fork's `main` when the KatWork work started): `768c0f972d6fce82224eb8db001094868a0c321a`
- Upstream `storytold/soundcraft` head at that time: `768c0f972d6fce82224eb8db001094868a0c321a`

## KatWork patches

| Patch | Upstream PR |
|---|---|
| `chore(katwork): remove the ArtCraft marks` | not proposed upstream (the brand terms let a fork that proposes changes keep the marks; this one is a distributed modified version) |
| `feat(katwork): plugin-hosts feature, on by default` | draft, not opened (in the fork PR body) |

## The `plugin-hosts` gate

The CLAP, VST3 and Audio Units hosts (`soundcraft-clap-host`, `soundcraft-vst3-host`, `soundcraft-au-host`) load native
plug-ins into the process with no sandbox. They are optional dependencies of `soundcraft-mix`, `soundcraft-engine` and
`soundcraft-ui-egui` behind the feature `plugin-hosts`, on by default, so an upstream build is unchanged. The workspace
inherits `soundcraft-mix`, `-engine`, `-playback`, `-automation` and `-ui-egui` with their defaults off, and every crate
that depends on them (`soundcraft-playback`, `soundcraft-automation`, `soundcraft-cli`, `soundcraft-web`, the desktop app)
forwards `plugin-hosts`, also on by default. The KatWork build is `--no-default-features`: then a `clap:`, `vst3:` or `au:`
plug-in id is an unknown plug-in, `engine.clap_plugins` and `engine.vst3_plugins` return `[]`, the Mix window lists no
hosted plug-ins, and a session that names one opens and plays without it. `engine.plugins` lists the built-in plug-ins
either way.

The UI of the hosts is hidden with the gate off (founder ruling: hide them): the Mix window's insert and instrument menus offer
the built-in plug-ins only (no CLAP, VST3 or Audio Units submenu, no "Scan CLAP" / "Scan VST3" button), Search (the command
palette) does not offer `engine.clap_plugins` or `engine.vst3_plugins`, and a hosted plug-in's editor buttons and window never
show (no host can load one). SoundCraft has no plug-in path setting. The two engine queries stay registered for automation and
answer `[]`, as the plan's test asks.

`scripts/katwork-profile.sh` (CI job `katwork-profile`) checks the rows of `scripts/katwork-profile.tsv`: the hosts are
not in the CLI or web build graph with the gate off, and are with the defaults; `crates/engine/tests/katwork_profile.rs`
runs the engine with the gate off.

## The marks

The ArtCraft name, wordmark and logos are trademarks of the ArtCraft Team and are not covered by the app's licence. This fork
removes them, as the brand terms require of a distributed modified version: `docs/brand/` is deleted; links to the ArtCraft
website and its community (Discord), and taglines that present the build as an ArtCraft product, are removed; help links and
`app.links`-style results point to the upstream repository (https://github.com/storytold/soundcraft); README.md, NOTICE and the
About window carry the "Based on" line above as plain text. No new logo is drawn: where a mark was shown, the app's name is shown
as plain text.

`scripts/check-no-marks.sh` (CI job `no-marks` in `.github/workflows/katwork.yml`) fails when a tracked file is a copy of a
fork-base brand file (`scripts/katwork-marks/hashes`), when a line names the marks or the ArtCraft website, when a line names
ArtCraft and is not in `scripts/katwork-marks/allow`, or when an image below is missing from this file.

### What still names ArtCraft, and why (`scripts/katwork-marks/allow`)

- Copyright notices (`LICENSE-MIT`, `NOTICE`, README): required by the licences.
- Application ids `ai.storyteller.soundcraft` and installer paths and folders (`Programs\ArtCraft\...`): kept unchanged; they are
  identifiers, not marks, and changing them would break upgrades of existing installs.
- The nightly channel's messages, comments and tests that name ArtCraft Launcher (`build_flavor.rs`, `main.rs`, `menus.rs`,
  `updates_ui.rs`, the About window's nightly-only "Updated by ArtCraft Launcher." line and its translation, the README's
  "Nightly" section) and the launcher's folders in `profile_root.rs` and the Flatpak permission
  (`xdg-data/ArtCraft/versions`): they name a separate program, the upstream nightly builds' updater; KatWork builds no nightly.
- The upstream release workflow's secret name (`release.yml`) and the Windows ARM64 workflow's install path: upstream CI
  configuration, unchanged; KatWork does not run it.
- The "Based on" line in the About window (`crates/ui-egui/src/panels/about.rs`).

## Image assets of the fork base

Removed (the brand files, deleted with `docs/brand/`):

- `docs/brand/LICENSE-brand.txt`: removed
- `docs/brand/artcraft-logo-white.png`: removed
- `docs/brand/artcraft-logo-white.svg`: removed
- `docs/brand/artcraft-logo.png`: removed
- `docs/brand/artcraft-logo.svg`: removed
- `docs/brand/artcraft-mark-black.png`: removed
- `docs/brand/artcraft-mark-black.svg`: removed
- `docs/brand/artcraft-mark.png`: removed
- `docs/brand/artcraft-mark.svg`: removed

Every other image of the fork base, each kept:

- `assets/app-icon/hicolor/128x128/apps/ai.storyteller.soundcraft.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/16x16/apps/ai.storyteller.soundcraft.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/24x24/apps/ai.storyteller.soundcraft.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/256x256/apps/ai.storyteller.soundcraft-nightly.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/256x256/apps/ai.storyteller.soundcraft.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/32x32/apps/ai.storyteller.soundcraft.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/48x48/apps/ai.storyteller.soundcraft.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/512x512/apps/ai.storyteller.soundcraft.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/64x64/apps/ai.storyteller.soundcraft.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/scalable/apps/ai.storyteller.soundcraft.svg`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/icon.svg`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/soundcraft-1024.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/soundcraft-macos-512.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/soundcraft-nightly-macos-512.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/soundcraft-nightly.icns`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/soundcraft-nightly.ico`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/soundcraft-nightly.png`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/soundcraft.icns`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `assets/app-icon/soundcraft.ico`: kept: placeholder per ATTRIBUTION.md (SoundCraft contributors, MIT OR Apache-2.0; no `assets/app-icon/LICENSE.txt` upstream; viewed: no ArtCraft mark)
- `docs/images/automation.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/edit.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/midi.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/mix.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/score.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/surround.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `docs/images/video.png`: kept: screenshot of the upstream build (viewed: no ArtCraft mark)
- `packaging/macos/dmg/background.svg`: kept: macOS disk-image background, original artwork per ATTRIBUTION.md (the app name and icon; source read: no ArtCraft mark)
- `packaging/macos/dmg/background.tiff`: kept: macOS disk-image background rendered from `background.svg` (the app name and icon; viewed: no ArtCraft mark)
