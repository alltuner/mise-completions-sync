# Changelog

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
