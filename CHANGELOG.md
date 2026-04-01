# Changelog

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
