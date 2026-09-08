# Changelog

## [0.5.16](https://github.com/alltuner/mise-completions-sync/compare/v0.5.15...v0.5.16) (2026-09-08)


### Features

* add nova completion ([#156](https://github.com/alltuner/mise-completions-sync/issues/156)) ([e92a7de](https://github.com/alltuner/mise-completions-sync/commit/e92a7debb42f8bc1850a62e613d33f366c8d3bf8))
* add tealdeer bundled completion support, and a `completion_name` override for tools whose binary differs from their mise name ([#151](https://github.com/alltuner/mise-completions-sync/issues/151)) ([99dfc54](https://github.com/alltuner/mise-completions-sync/commit/99dfc5496b084fbad8426a48157f965cc6f94c18))
* add trash-cli ([#146](https://github.com/alltuner/mise-completions-sync/issues/146)) ([a44c74e](https://github.com/alltuner/mise-completions-sync/commit/a44c74e023fe2ce7e1fec461d0c9e0efbe762fdd))
* **registry:** add forgejo-cli ([#118](https://github.com/alltuner/mise-completions-sync/issues/118)) ([fe4b967](https://github.com/alltuner/mise-completions-sync/commit/fe4b967264a5cc1581e831bd56a83f82c0b9c302))
* **registry:** add hubble ([#153](https://github.com/alltuner/mise-completions-sync/issues/153)) ([a2773e2](https://github.com/alltuner/mise-completions-sync/commit/a2773e22a32cfc3bedcd1b73a90bb31ac1a29e9c))
* **registry:** add yazi ([#147](https://github.com/alltuner/mise-completions-sync/issues/147)) ([9b2613b](https://github.com/alltuner/mise-completions-sync/commit/9b2613b0af88e36bbdd075d36326173d7dade11d))
* **site:** publish through the fleet's registry instead of GitHub Pages ([#154](https://github.com/alltuner/mise-completions-sync/issues/154)) ([acc4a1f](https://github.com/alltuner/mise-completions-sync/commit/acc4a1fca34ea2bf79b43c2d864d8f1e18b6dab1))


### Bug Fixes

* **deps:** update rust crate dirs to v7 ([#157](https://github.com/alltuner/mise-completions-sync/issues/157)) ([44eb67f](https://github.com/alltuner/mise-completions-sync/commit/44eb67fd79e4ae42fd39ed8ecc476cf578c04239))
* write completions under the command name, not the mise tool name ([#158](https://github.com/alltuner/mise-completions-sync/issues/158)) ([cfe8883](https://github.com/alltuner/mise-completions-sync/commit/cfe8883c959afe3c1d8026b74a2a6d9f2b605b1b))


### Miscellaneous Chores

* **deps:** update astral-sh/setup-uv action to v10 ([#152](https://github.com/alltuner/mise-completions-sync/issues/152)) ([d68ca2c](https://github.com/alltuner/mise-completions-sync/commit/d68ca2c156ef36fc090db98714d1d971589e8989))
* **deps:** update rust dependencies ([#149](https://github.com/alltuner/mise-completions-sync/issues/149)) ([fb05a2d](https://github.com/alltuner/mise-completions-sync/commit/fb05a2de919e8107a82eec1326f4819e46de4171))
* drop the GitHub Pages workflow ([#155](https://github.com/alltuner/mise-completions-sync/issues/155)) ([266b19c](https://github.com/alltuner/mise-completions-sync/commit/266b19c4a68a35a49e46e2a06f7ef4a396eb9ea4))


### Documentation Updates

* point at the canonical docs domain ([#143](https://github.com/alltuner/mise-completions-sync/issues/143)) ([f81c5c7](https://github.com/alltuner/mise-completions-sync/commit/f81c5c7870ff7d0e23a601c9331045f6dee90069))

## [0.5.15](https://github.com/alltuner/mise-completions-sync/compare/v0.5.14...v0.5.15) (2026-07-29)


### Features

* add 14 new tool patterns to registry ([#88](https://github.com/alltuner/mise-completions-sync/issues/88)) ([4ad153b](https://github.com/alltuner/mise-completions-sync/commit/4ad153be3517323ae79105c8ba8786f55a22925a))

## [0.5.14](https://github.com/alltuner/mise-completions-sync/compare/v0.5.13...v0.5.14) (2026-07-29)


### Features

* **registry:** merge a user registry over the built-in one ([#131](https://github.com/alltuner/mise-completions-sync/issues/131)) ([e62224f](https://github.com/alltuner/mise-completions-sync/commit/e62224faa159fcbcddd0f81fac5238babc442cf4))
* **registry:** support tools shipping completion files ([#136](https://github.com/alltuner/mise-completions-sync/issues/136)) ([d51283b](https://github.com/alltuner/mise-completions-sync/commit/d51283b822017141237bf0d0a70e8005f769b775)), closes [#101](https://github.com/alltuner/mise-completions-sync/issues/101)


### Bug Fixes

* **registry:** correct entries the audit found broken ([#141](https://github.com/alltuner/mise-completions-sync/issues/141)) ([07e7a55](https://github.com/alltuner/mise-completions-sync/commit/07e7a555b6f783cbe07218d596aae5e0f55af9e6))


### Documentation Updates

* describe bundled entries in the registry reference ([#139](https://github.com/alltuner/mise-completions-sync/issues/139)) ([3140829](https://github.com/alltuner/mise-completions-sync/commit/3140829f0551ba569ff207565258a9c0e812f39a))


### CI/CD Changes

* audit the whole registry on a schedule ([#138](https://github.com/alltuner/mise-completions-sync/issues/138)) ([087e01d](https://github.com/alltuner/mise-completions-sync/commit/087e01d990581180803761c801fa9226fe4230f3))

## [0.5.13](https://github.com/alltuner/mise-completions-sync/compare/v0.5.12...v0.5.13) (2026-07-29)


### Features

* **registry:** add uvx completions ([#132](https://github.com/alltuner/mise-completions-sync/issues/132)) ([c26e3a5](https://github.com/alltuner/mise-completions-sync/commit/c26e3a58776f927edeb72fd33f4593beacf17e73)), closes [#76](https://github.com/alltuner/mise-completions-sync/issues/76)


### Bug Fixes

* **registry:** correct mdbook and drop tools without completions ([#134](https://github.com/alltuner/mise-completions-sync/issues/134)) ([07ef18d](https://github.com/alltuner/mise-completions-sync/commit/07ef18d423cf1b515a52948b009b04a26cbe3269))


### CI/CD Changes

* publish to crates.io via OIDC trusted publishing ([#135](https://github.com/alltuner/mise-completions-sync/issues/135)) ([cdbb476](https://github.com/alltuner/mise-completions-sync/commit/cdbb47625ff9f44efd841b0c8b928ebc4d5bf9e8))

## [0.5.12](https://github.com/alltuner/mise-completions-sync/compare/v0.5.11...v0.5.12) (2026-07-28)


### Features

* add ipython, patool, ratarmount ([#108](https://github.com/alltuner/mise-completions-sync/issues/108)) ([5ceda4e](https://github.com/alltuner/mise-completions-sync/commit/5ceda4e331181202ae5f4e5d0a3f3f6d97332f45))
* **registry:** support companion binaries ([#119](https://github.com/alltuner/mise-completions-sync/issues/119)) ([663e8a1](https://github.com/alltuner/mise-completions-sync/commit/663e8a1bd83935a9b0a150df46615e58dde5e1e2))
* **sync-with-flags:** Allow flags --global --local --current ([#103](https://github.com/alltuner/mise-completions-sync/issues/103)) ([469e900](https://github.com/alltuner/mise-completions-sync/commit/469e90016ef60733ddb975aaab92ccf5bdc4a0b3))


### Code Refactoring

* **sync:** fail loudly on empty completion output ([#114](https://github.com/alltuner/mise-completions-sync/issues/114)) ([b663a7b](https://github.com/alltuner/mise-completions-sync/commit/b663a7b76b63183e4880ff31b34476eeedac2d4b))


### Miscellaneous Chores

* **deps:** update rust crate toml to v1.1.4 ([#128](https://github.com/alltuner/mise-completions-sync/issues/128)) ([04d81d2](https://github.com/alltuner/mise-completions-sync/commit/04d81d2c8d099fe8fd5ea81c02eadec2d3c2a76e))
* remove unused release-please config ([#129](https://github.com/alltuner/mise-completions-sync/issues/129)) ([7467e04](https://github.com/alltuner/mise-completions-sync/commit/7467e047aac6fc6da3f464c2b105d7e15e6a8120))

## [0.5.11](https://github.com/alltuner/mise-completions-sync/compare/v0.5.10...v0.5.11) (2026-07-28)


### Features

* add new tools to completion registry ([#112](https://github.com/alltuner/mise-completions-sync/issues/112)) ([4310554](https://github.com/alltuner/mise-completions-sync/commit/4310554dda76bdff5f1925835e34ff4e03b37635))
* add support for ast-grep ([#100](https://github.com/alltuner/mise-completions-sync/issues/100)) ([ed10974](https://github.com/alltuner/mise-completions-sync/commit/ed109741f3d855066dbe2172937d76f416601e25))
* **registry:** add requires for tools needing a helper binary ([#126](https://github.com/alltuner/mise-completions-sync/issues/126)) ([476e5e9](https://github.com/alltuner/mise-completions-sync/commit/476e5e90ebd78570e513a62e8ec9677311dfd9c4))


### Bug Fixes

* **registry:** correct xh completion command ([#113](https://github.com/alltuner/mise-completions-sync/issues/113)) ([38401b7](https://github.com/alltuner/mise-completions-sync/commit/38401b7cbc2fd11cf0914fbae76066c9bbfecd89))
* **registry:** generate fnox completions via usage ([#124](https://github.com/alltuner/mise-completions-sync/issues/124)) ([8c821b1](https://github.com/alltuner/mise-completions-sync/commit/8c821b1eed1c0436d1bb34bb55694ec796ff4477))
* **registry:** use right name for flux ([#121](https://github.com/alltuner/mise-completions-sync/issues/121)) ([4a0f671](https://github.com/alltuner/mise-completions-sync/commit/4a0f67177169d0ea6a8b2380c20b44c953c026f9))
* **sync:** report stderr from commands that succeed ([#127](https://github.com/alltuner/mise-completions-sync/issues/127)) ([1d6e7d7](https://github.com/alltuner/mise-completions-sync/commit/1d6e7d72b98593890099a16023583538ab677f70))


### Miscellaneous Chores

* **deps:** update actions/checkout action to v7 ([#110](https://github.com/alltuner/mise-completions-sync/issues/110)) ([4b58e6f](https://github.com/alltuner/mise-completions-sync/commit/4b58e6f06b3e030a1a285d7d946525f61927253f))
* **deps:** update astral-sh/setup-uv action to v9 ([#120](https://github.com/alltuner/mise-completions-sync/issues/120)) ([1c77676](https://github.com/alltuner/mise-completions-sync/commit/1c77676558a71a19584bb24e37d8ad126c2c602e))
* **deps:** update houseabsolute/actions-rust-cross action to v1.0.8 ([#111](https://github.com/alltuner/mise-completions-sync/issues/111)) ([b6cd8b0](https://github.com/alltuner/mise-completions-sync/commit/b6cd8b0b8e392091f85582e190ac038751a46bda))
* **deps:** update rust dependencies ([#116](https://github.com/alltuner/mise-completions-sync/issues/116)) ([1f7c534](https://github.com/alltuner/mise-completions-sync/commit/1f7c534f40afd29ff4f136eabdf9b2eaf0b4d9fb))


### CI/CD Changes

* check docs/tools.md lists the same tools as registry.toml ([#125](https://github.com/alltuner/mise-completions-sync/issues/125)) ([3b66c7d](https://github.com/alltuner/mise-completions-sync/commit/3b66c7dd37eb4b777c3f7a8916acad52f6824169))
* run claude review on renovate PRs and skip forks ([#123](https://github.com/alltuner/mise-completions-sync/issues/123)) ([f67fdad](https://github.com/alltuner/mise-completions-sync/commit/f67fdadf397b9d2a1d2c65e34572b1a12ce39c4c))

## [0.5.10](https://github.com/alltuner/mise-completions-sync/compare/v0.5.9...v0.5.10) (2026-06-09)


### Features

* add --new-only flag ([#83](https://github.com/alltuner/mise-completions-sync/issues/83)) ([26ae9ca](https://github.com/alltuner/mise-completions-sync/commit/26ae9caf25fc890c92067c4b6eb746fbbaa67865))
* **pipx:** add fish support ([#98](https://github.com/alltuner/mise-completions-sync/issues/98)) ([7c329df](https://github.com/alltuner/mise-completions-sync/commit/7c329df376ef174debfe1eb5bc4a05b3b647dfd0))


### Bug Fixes

* **registry:** correct restic completion command ([#107](https://github.com/alltuner/mise-completions-sync/issues/107)) ([019e8d1](https://github.com/alltuner/mise-completions-sync/commit/019e8d1165c6dcfdd597a2086a7e6d1866a6f00f))
* **registry:** remove kubectx (closes [#104](https://github.com/alltuner/mise-completions-sync/issues/104)) ([#106](https://github.com/alltuner/mise-completions-sync/issues/106)) ([57944a5](https://github.com/alltuner/mise-completions-sync/commit/57944a5e52072a1eec91c8abca9388a394c74411))


### Miscellaneous Chores

* **deps:** update astral-sh/setup-uv action to v8.2.0 ([#102](https://github.com/alltuner/mise-completions-sync/issues/102)) ([8d3d29a](https://github.com/alltuner/mise-completions-sync/commit/8d3d29aaf404b7b5793a1aa95d557909dd3ee206))

## [0.5.9](https://github.com/alltuner/mise-completions-sync/compare/v0.5.8...v0.5.9) (2026-05-27)


### Bug Fixes

* **registry:** remove fzf and zoxide (closes [#78](https://github.com/alltuner/mise-completions-sync/issues/78)) ([#96](https://github.com/alltuner/mise-completions-sync/issues/96)) ([8f3f1d2](https://github.com/alltuner/mise-completions-sync/commit/8f3f1d2471691472c898821e89b4286ca2f4b21c))

## [0.5.8](https://github.com/alltuner/mise-completions-sync/compare/v0.5.7...v0.5.8) (2026-05-27)


### Features

* add self-completion subcommand (closes [#75](https://github.com/alltuner/mise-completions-sync/issues/75)) ([#94](https://github.com/alltuner/mise-completions-sync/issues/94)) ([8521f7c](https://github.com/alltuner/mise-completions-sync/commit/8521f7c1f50605085ff3c852469dbe4dc9169a57))
* add talosctl ([#95](https://github.com/alltuner/mise-completions-sync/issues/95)) ([a43f446](https://github.com/alltuner/mise-completions-sync/commit/a43f446f137f76690837bd0e17a040e9f9107d4d))


### Bug Fixes

* **ci:** restore x86_64-unknown-linux-musl release binary ([#90](https://github.com/alltuner/mise-completions-sync/issues/90)) ([0dccee5](https://github.com/alltuner/mise-completions-sync/commit/0dccee5f90df29ac58825515596ed72028f5f449)), closes [#89](https://github.com/alltuner/mise-completions-sync/issues/89)
* use standard pattern for lefthook (closes [#86](https://github.com/alltuner/mise-completions-sync/issues/86)) ([#91](https://github.com/alltuner/mise-completions-sync/issues/91)) ([48b5a51](https://github.com/alltuner/mise-completions-sync/commit/48b5a5175f1d3e840a66a230a52338a987461f26))


### Miscellaneous Chores

* **deps:** update rust crate serde_json to v1.0.150 ([#87](https://github.com/alltuner/mise-completions-sync/issues/87)) ([2581de9](https://github.com/alltuner/mise-completions-sync/commit/2581de9137a55930358c7779a07d98122cf86d0a))

## [0.5.7](https://github.com/alltuner/mise-completions-sync/compare/v0.5.6...v0.5.7) (2026-05-12)


### Features

* add support for tree-sitter ([#77](https://github.com/alltuner/mise-completions-sync/issues/77)) ([afd6605](https://github.com/alltuner/mise-completions-sync/commit/afd660584746e717fdaab9aa7238ce2e571ab305))
* allow overriding output dirs with ENV vars ([#52](https://github.com/alltuner/mise-completions-sync/issues/52)) ([fc423c9](https://github.com/alltuner/mise-completions-sync/commit/fc423c9bd3c610efc69bf97032eaeacb8a047536))
* support tools from alternative backends ([#41](https://github.com/alltuner/mise-completions-sync/issues/41)) ([6e39bc0](https://github.com/alltuner/mise-completions-sync/commit/6e39bc08bf30cf8aba8124591b0f957d10fb4b17))


### Bug Fixes

* merge duplicate tests module in sync.rs ([#82](https://github.com/alltuner/mise-completions-sync/issues/82)) ([c469875](https://github.com/alltuner/mise-completions-sync/commit/c4698753d03a5216e3e7efa34c1bf3c107affe72))
* rename installed binary to misecompsync (closes [#79](https://github.com/alltuner/mise-completions-sync/issues/79)) ([#81](https://github.com/alltuner/mise-completions-sync/issues/81)) ([bd235ff](https://github.com/alltuner/mise-completions-sync/commit/bd235ffd075f86c6cc19ed0b011265a6d07612f9))

## [0.5.6](https://github.com/alltuner/mise-completions-sync/compare/v0.5.5...v0.5.6) (2026-05-04)


### Documentation Updates

* standardize README to alltuner brand structure ([#73](https://github.com/alltuner/mise-completions-sync/issues/73)) ([2057074](https://github.com/alltuner/mise-completions-sync/commit/2057074e75141b738aadb3cd323a01ef60856999))

## [0.5.5](https://github.com/alltuner/mise-completions-sync/compare/v0.5.4...v0.5.5) (2026-05-03)


### Miscellaneous Chores

* **deps:** update rust dependencies ([#71](https://github.com/alltuner/mise-completions-sync/issues/71)) ([ba67f02](https://github.com/alltuner/mise-completions-sync/commit/ba67f02946eeb1815daa8d54e6b27d2dcd9964fe))

## [0.5.4](https://github.com/alltuner/mise-completions-sync/compare/v0.5.3...v0.5.4) (2026-05-03)


### Build System

* track Cargo.lock so release builds with --locked succeed ([#69](https://github.com/alltuner/mise-completions-sync/issues/69)) ([78a9a78](https://github.com/alltuner/mise-completions-sync/commit/78a9a7852b46e707b5ae95d6d92628d0bfb52f38))

## [0.5.3](https://github.com/alltuner/mise-completions-sync/compare/v0.5.2...v0.5.3) (2026-05-03)


### CI/CD Changes

* align PR title check with org canonical workflow ([#67](https://github.com/alltuner/mise-completions-sync/issues/67)) ([73a1093](https://github.com/alltuner/mise-completions-sync/commit/73a10931a2c97106d5a8471be262606a9df4d714))

## [0.5.2](https://github.com/alltuner/mise-completions-sync/compare/v0.5.1...v0.5.2) (2026-05-03)


### Miscellaneous Chores

* **deps:** update amannn/action-semantic-pull-request action to v6 ([#66](https://github.com/alltuner/mise-completions-sync/issues/66)) ([be5454f](https://github.com/alltuner/mise-completions-sync/commit/be5454f69cd32758b59275e990e5228b3af65c27))


### CI/CD Changes

* validate PR titles as conventional commits ([#64](https://github.com/alltuner/mise-completions-sync/issues/64)) ([f27297d](https://github.com/alltuner/mise-completions-sync/commit/f27297def7e61488e24f54ea6f5814a475f43528))

## [0.5.1](https://github.com/alltuner/mise-completions-sync/compare/v0.5.0...v0.5.1) (2026-05-02)


### Bug Fixes

* bundle dependency and registry updates from recent merges ([#59](https://github.com/alltuner/mise-completions-sync/issues/59)) ([98e9409](https://github.com/alltuner/mise-completions-sync/commit/98e9409a7df5d32810ad0d41e58fb046a6926567))
* **registry:** remove mkcert (closes [#50](https://github.com/alltuner/mise-completions-sync/issues/50)) ([#61](https://github.com/alltuner/mise-completions-sync/issues/61)) ([b588589](https://github.com/alltuner/mise-completions-sync/commit/b588589f06c413028438ec9faf367879f29c223b))

## [0.5.0](https://github.com/alltuner/mise-completions-sync/compare/v0.4.4...v0.5.0) (2026-04-01)


### Features

* add new tools ([#40](https://github.com/alltuner/mise-completions-sync/issues/40)) ([4617152](https://github.com/alltuner/mise-completions-sync/commit/461715210f3123aac1ec2471ecb334c0d232808a))
* add prek completion support ([#48](https://github.com/alltuner/mise-completions-sync/issues/48)) ([b2bb8a9](https://github.com/alltuner/mise-completions-sync/commit/b2bb8a982e35c8bf5d08ad3cf15538fa2d9c2510)), closes [#36](https://github.com/alltuner/mise-completions-sync/issues/36)


### Bug Fixes

* **deps:** update rust crate toml to v1 ([#35](https://github.com/alltuner/mise-completions-sync/issues/35)) ([ea3241f](https://github.com/alltuner/mise-completions-sync/commit/ea3241fa003e8c5a60b0c70d07e22cb9f9de8239))
* **generate-registry:** migrate from mise registry.toml to API endpoint ([#42](https://github.com/alltuner/mise-completions-sync/issues/42)) ([329a9b9](https://github.com/alltuner/mise-completions-sync/commit/329a9b97754df3e264ea9d47ce11bbe21a02406e))
* remove unused json import in generate-registry ([#47](https://github.com/alltuner/mise-completions-sync/issues/47)) ([d3e7c29](https://github.com/alltuner/mise-completions-sync/commit/d3e7c29a63751b203108bfa12884156ff5a03b5d))

## [0.4.4](https://github.com/alltuner/mise-completions-sync/compare/v0.4.3...v0.4.4) (2026-02-11)


### Bug Fixes

* remove gcloud from registry ([#32](https://github.com/alltuner/mise-completions-sync/issues/32)) ([#33](https://github.com/alltuner/mise-completions-sync/issues/33)) ([fa296f6](https://github.com/alltuner/mise-completions-sync/commit/fa296f69e5812a0e83c081ec7431d02282681662))

## [0.4.3](https://github.com/alltuner/mise-completions-sync/compare/v0.4.2...v0.4.3) (2026-01-16)


### Bug Fixes

* **ci:** remove auto-generation of tools docs from workflow ([#29](https://github.com/alltuner/mise-completions-sync/issues/29)) ([25d873b](https://github.com/alltuner/mise-completions-sync/commit/25d873b7ea502e9f3cb21a0e21120b88e906060b))

## [0.4.2](https://github.com/alltuner/mise-completions-sync/compare/v0.4.1...v0.4.2) (2026-01-15)


### Bug Fixes

* **ci:** remove auto-generation of tools docs from workflow ([#28](https://github.com/alltuner/mise-completions-sync/issues/28)) ([9cdc1af](https://github.com/alltuner/mise-completions-sync/commit/9cdc1afe9a5c0fb1d0d3b6054fb9b7d131a58ae2))
* **scripts:** handle missing mise in CI gracefully ([#26](https://github.com/alltuner/mise-completions-sync/issues/26)) ([70651da](https://github.com/alltuner/mise-completions-sync/commit/70651daf251e6248f6ff58812fb21f0dcd7c451c))

## [0.4.1](https://github.com/alltuner/mise-completions-sync/compare/v0.4.0...v0.4.1) (2026-01-15)


### Bug Fixes

* **docs:** trigger rebuild on CHANGELOG changes and include in site ([#24](https://github.com/alltuner/mise-completions-sync/issues/24)) ([d1a5bd1](https://github.com/alltuner/mise-completions-sync/commit/d1a5bd1695f9a2d44b8e151475051b9df4918086))

## [0.4.0](https://github.com/alltuner/mise-completions-sync/compare/v0.3.0...v0.4.0) (2026-01-15)


### Features

* **docs:** enhance tool documentation with metadata and status tracking ([#23](https://github.com/alltuner/mise-completions-sync/issues/23)) ([8159951](https://github.com/alltuner/mise-completions-sync/commit/81599513a5e3f689e6b52d09ed0530ecc7dd68bb))


### Bug Fixes

* **registry:** remove tools without completion support ([#21](https://github.com/alltuner/mise-completions-sync/issues/21)) ([4d5e7ba](https://github.com/alltuner/mise-completions-sync/commit/4d5e7ba585cd80b0b70eabc4890909999318295d))

## [0.3.0](https://github.com/alltuner/mise-completions-sync/compare/v0.2.2...v0.3.0) (2026-01-15)


### Features

* add worktrees directory to gitignore ([#15](https://github.com/alltuner/mise-completions-sync/issues/15)) ([c43f8ae](https://github.com/alltuner/mise-completions-sync/commit/c43f8aee52cc323ae6a75305ff13c18025a10c44))
* **docs:** convert from mdBook to MkDocs with Material theme ([#18](https://github.com/alltuner/mise-completions-sync/issues/18)) ([1075920](https://github.com/alltuner/mise-completions-sync/commit/1075920b4285c1373a74aafcd5dafbb0fc50beb1))
* **registry:** pattern-based format with schema versioning ([#17](https://github.com/alltuner/mise-completions-sync/issues/17)) ([e7e72cc](https://github.com/alltuner/mise-completions-sync/commit/e7e72cc86fe1a6ee508dfda625156443d3b3967b))

## [0.2.2](https://github.com/alltuner/mise-completions-sync/compare/v0.2.1...v0.2.2) (2026-01-15)


### Bug Fixes

* **registry:** remove tools without completion support ([#14](https://github.com/alltuner/mise-completions-sync/issues/14)) ([7f3a0b5](https://github.com/alltuner/mise-completions-sync/commit/7f3a0b57bc084eebac2bba345bcaa892965c1165))
* wrap completion commands with mise x for tool availability ([#12](https://github.com/alltuner/mise-completions-sync/issues/12)) ([32f58c5](https://github.com/alltuner/mise-completions-sync/commit/32f58c509744f7e91c4c805c6e69caf322418616))

## [0.2.1](https://github.com/alltuner/mise-completions-sync/compare/v0.2.0...v0.2.1) (2026-01-15)


### Bug Fixes

* **ci:** chain release workflows and always bump patch ([#9](https://github.com/alltuner/mise-completions-sync/issues/9)) ([415f670](https://github.com/alltuner/mise-completions-sync/commit/415f670336f716cfe30c65a3e4174d9e1c15c71d))

## [0.2.0](https://github.com/alltuner/mise-completions-sync/compare/v0.1.0...v0.2.0) (2026-01-15)


### Features

* add release infrastructure and expand registry ([#5](https://github.com/alltuner/mise-completions-sync/issues/5)) ([e36dd40](https://github.com/alltuner/mise-completions-sync/commit/e36dd4046be63cd8b72ed1697601907e473bda4a))
