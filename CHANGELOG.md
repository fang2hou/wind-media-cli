## 0.1.1 - 2026-04-23
#### 🐛 Bug Fixes
- (**ci**) use CHANGELOG.md as source of truth for release notes (#12) - (68b0c2e) - Zhou Fang

## [0.2.1](https://github.com/fang2hou/wind-media-cli/compare/v0.2.0...v0.2.1) (2026-04-22)


### Bug Fixes

* **ci:** add checkout step to publish job for gh CLI ([#18](https://github.com/fang2hou/wind-media-cli/issues/18)) ([f887f97](https://github.com/fang2hou/wind-media-cli/commit/f887f97c16f8a907a08d79f5e317ad8ebc544163))

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
