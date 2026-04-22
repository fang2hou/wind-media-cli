## 0.1.1 - 2026-04-23
#### 🐛 Bug Fixes
- (**ci**) use CHANGELOG.md as source of truth for release notes (#12) - (68b0c2e) - Zhou Fang

## [0.3.0](https://github.com/fang2hou/wind-media-cli/compare/wind-media-cli-v0.2.0...wind-media-cli-v0.3.0) (2026-04-22)


### ✨ Features

* add configurable max_backups support ([#1](https://github.com/fang2hou/wind-media-cli/issues/1)) ([bb385ed](https://github.com/fang2hou/wind-media-cli/commit/bb385ed6e620e9717d4525479841105612e4de11))
* add nushell completions, mise install support, and improve Windows config ([c511f95](https://github.com/fang2hou/wind-media-cli/commit/c511f95bc4689e2c7b22f709ab1437c785d39969))
* add project docs, CI polish, and pre-commit hooks ([8a747e2](https://github.com/fang2hou/wind-media-cli/commit/8a747e22001e338e176fc88e69a0ba9f73101459))
* **ci:** replace cocogitto release with release-please ([#14](https://github.com/fang2hou/wind-media-cli/issues/14)) ([814c102](https://github.com/fang2hou/wind-media-cli/commit/814c1022566c98c120fafc9c354ed34976b419d7))
* initial release of wind-media-cli ([34a7a7a](https://github.com/fang2hou/wind-media-cli/commit/34a7a7a3fe18deee8131b7136386b1a55fbf1dc4))
* upgrade to Rust 1.95 + address 14 review findings ([0b15517](https://github.com/fang2hou/wind-media-cli/commit/0b155175621ef48d8ab5df8ffa112b78c8770164))


### 🐛 Bug Fixes

* add release-please accumulation test marker ([#17](https://github.com/fang2hou/wind-media-cli/issues/17)) ([b896195](https://github.com/fang2hou/wind-media-cli/commit/b896195008013b82938445b02d1cfb51bb668e7f))
* **ci:** add checkout step to publish job for gh CLI ([#18](https://github.com/fang2hou/wind-media-cli/issues/18)) ([f887f97](https://github.com/fang2hou/wind-media-cli/commit/f887f97c16f8a907a08d79f5e317ad8ebc544163))
* **ci:** add merge commit type to cog.toml ([#7](https://github.com/fang2hou/wind-media-cli/issues/7)) ([de4689e](https://github.com/fang2hou/wind-media-cli/commit/de4689e02a85d8f33b38e755df93eb2423d2e2cc))
* **ci:** add pull-requests write permission for release-please ([#15](https://github.com/fang2hou/wind-media-cli/issues/15)) ([073491c](https://github.com/fang2hou/wind-media-cli/commit/073491c3d290628fdfcc49d7ec9c5eff2d887f84))
* **ci:** fix malformed JSON in release-please-config.json ([#24](https://github.com/fang2hou/wind-media-cli/issues/24)) ([38e6d1f](https://github.com/fang2hou/wind-media-cli/commit/38e6d1fc610c216b3c7746998d6cd400a2903161))
* **ci:** move changelog-sections to per-package config level ([#26](https://github.com/fang2hou/wind-media-cli/issues/26)) ([c984b14](https://github.com/fang2hou/wind-media-cli/commit/c984b14ff2eb133f7e1c66588d01c38094673542))
* **ci:** remove duplicate clippy job and toolchain override ([6f91307](https://github.com/fang2hou/wind-media-cli/commit/6f91307e669ffeef46ab5fe1ff0c47f7adbcc3b7))
* **ci:** remove release-type override from action input ([#28](https://github.com/fang2hou/wind-media-cli/issues/28)) ([4acf650](https://github.com/fang2hou/wind-media-cli/commit/4acf6503dc40f6e52105d5c7fd7cafb286bd4821))
* **ci:** remove trailing duplicate braces from release-please-config.json ([#29](https://github.com/fang2hou/wind-media-cli/issues/29)) ([3ee1d35](https://github.com/fang2hou/wind-media-cli/commit/3ee1d35e8623b58a3c0065d4f38f4d700b74cf87))
* **ci:** remove Windows ARM target from release workflow ([#11](https://github.com/fang2hou/wind-media-cli/issues/11)) ([687b86d](https://github.com/fang2hou/wind-media-cli/commit/687b86d0cbe43f621d9c782f1665a4dc6a802396))
* **ci:** skip strip for cross-compiled targets in release ([#10](https://github.com/fang2hou/wind-media-cli/issues/10)) ([5efb5fd](https://github.com/fang2hou/wind-media-cli/commit/5efb5fdc9218db92b469512612b234d60f076b7c))
* **ci:** specify toolchain version for dtolnay/rust-toolchain ([1820daa](https://github.com/fang2hou/wind-media-cli/commit/1820daa3924f0e6cdaca5b2fe2fc3bd853165648))
* **ci:** use CHANGELOG.md as source of truth for release notes ([#12](https://github.com/fang2hou/wind-media-cli/issues/12)) ([68b0c2e](https://github.com/fang2hou/wind-media-cli/commit/68b0c2e13d70b29c26795ac8d84b431669ee51a8))
* **ci:** use GitHub App token for release-please ([#22](https://github.com/fang2hou/wind-media-cli/issues/22)) ([3005640](https://github.com/fang2hou/wind-media-cli/commit/30056403a9ea583bd8ecd49b75d056e6f3f8204e))
* **ci:** use rust-toolchain.toml version instead of stable ([9d6c541](https://github.com/fang2hou/wind-media-cli/commit/9d6c5415ac1720429ee01d54e7fd38a64bd881e4))
* show full UUIDs in list, improve error handling and validation ([fa53eb9](https://github.com/fang2hou/wind-media-cli/commit/fa53eb9df2335c71589ae4bb0f736b1c8c61fed5))
* show full UUIDs, improve error handling and validation ([89dae30](https://github.com/fang2hou/wind-media-cli/commit/89dae3047690eb6ecc177685c6d80379e405a175))
* update SharedMedia link and remove trailing period from License ([#5](https://github.com/fang2hou/wind-media-cli/issues/5)) ([c21636b](https://github.com/fang2hou/wind-media-cli/commit/c21636bcc619a2e70d7ebf57c1d21749bab3da6b))
* use ~/.config on Windows instead of %APPDATA% ([ee3535a](https://github.com/fang2hou/wind-media-cli/commit/ee3535a1275efe84c2ec0f7610c4c8ef28560311))
* use ~/.config on Windows instead of %APPDATA% ([#3](https://github.com/fang2hou/wind-media-cli/issues/3)) ([e8eca07](https://github.com/fang2hou/wind-media-cli/commit/e8eca0751bdce0248837b91b11c737d0e2bc0412))
* use PathBuf::join for cross-platform path resolution ([#6](https://github.com/fang2hou/wind-media-cli/issues/6)) ([1d895d2](https://github.com/fang2hou/wind-media-cli/commit/1d895d2ef5e1fb428b64e6b9f0c54ad2be9ac244))
* use XDG config dir on all platforms ([#2](https://github.com/fang2hou/wind-media-cli/issues/2)) ([a65f1f1](https://github.com/fang2hou/wind-media-cli/commit/a65f1f1560b7c65a3c0ab359a819d49965ccb38d))

## [0.2.0](https://github.com/fang2hou/wind-media-cli/compare/0.1.0...v0.2.0) (2026-04-22)


### Features

* **ci:** replace cocogitto release with release-please ([#14](https://github.com/fang2hou/wind-media-cli/issues/14)) ([814c102](https://github.com/fang2hou/wind-media-cli/commit/814c1022566c98c120fafc9c354ed34976b419d7))


### Bug Fixes

* add release-please accumulation test marker ([#17](https://github.com/fang2hou/wind-media-cli/issues/17)) ([b896195](https://github.com/fang2hou/wind-media-cli/commit/b896195008013b82938445b02d1cfb51bb668e7f))
* **ci:** add pull-requests write permission for release-please ([#15](https://github.com/fang2hou/wind-media-cli/issues/15)) ([073491c](https://github.com/fang2hou/wind-media-cli/commit/073491c3d290628fdfcc49d7ec9c5eff2d887f84))
* **ci:** use CHANGELOG.md as source of truth for release notes ([#12](https://github.com/fang2hou/wind-media-cli/issues/12)) ([68b0c2e](https://github.com/fang2hou/wind-media-cli/commit/68b0c2e13d70b29c26795ac8d84b431669ee51a8))

## 0.1.0 - 2026-04-22
#### ✨ Features
- add nushell completions, mise install support, and improve Windows config - (c511f95) - Zhou Fang
- add configurable max_backups support (#1) - (bb385ed) - Zhou Fang
- add project docs, CI polish, and pre-commit hooks - (8a747e2) - Zhou Fang
- upgrade to Rust 1.95 + address 14 review findings - (0b15517) - Zhou Fang
- initial release of wind-media-cli - (34a7a7a) - Zhou Fang
#### 🐛 Bug Fixes
- (**ci**) remove Windows ARM target from release workflow (#11) - (687b86d) - Zhou Fang
- (**ci**) skip strip for cross-compiled targets in release (#10) - (5efb5fd) - Zhou Fang
- (**ci**) add merge commit type to cog.toml (#7) - (de4689e) - Zhou Fang
- (**ci**) specify toolchain version for dtolnay/rust-toolchain - (1820daa) - Zhou Fang
- (**ci**) remove duplicate clippy job and toolchain override - (6f91307) - Zhou Fang
- (**ci**) use rust-toolchain.toml version instead of stable - (9d6c541) - Zhou Fang
- use PathBuf::join for cross-platform path resolution (#6) - (1d895d2) - Zhou Fang
- update SharedMedia link and remove trailing period from License (#5) - (c21636b) - Zhou Fang
- show full UUIDs in list, improve error handling and validation - (fa53eb9) - Zhou Fang
- use ~/.config on Windows instead of %APPDATA% (#3) - (e8eca07) - Zhou Fang
- use ~/.config on Windows instead of %APPDATA% - (ee3535a) - Zhou Fang
- use XDG config dir on all platforms (#2) - (a65f1f1) - Zhou Fang
#### 📝 Documentation
- add emoji to changelog section headers - (29b4c11) - Zhou Fang
#### 👷 CI
- add release infrastructure and unify toolchain management - (887f669) - Zhou Fang
#### ♻️ Refactoring
- rename parse_csv to split_comma_list, reject whitespace - (0f235a1) - Zhou Fang
