# CHANGELOG

## 2.5.0 (2023-12-22)

### Added

- [#22](https://github.com/arkedge/c2a-tlmcmddb/pull/22): Import tlmcmddb v2.4.1
  - [ut-issl/tlm-cmd-db](https://github.com/ut-issl/tlm-cmd-db/) の [v2.4.1](https://github.com/ut-issl/tlm-cmd-db/releases/tag/v2.4.1) を fork し，サブディレクトリとして吸収合併した

### Documentation

- [#33](https://github.com/arkedge/c2a-tlmcmddb/pull/33): Add changelog

### Internal

- [#8](https://github.com/arkedge/c2a-tlmcmddb/pull/8): Update Rust crate serde to 1.0.164
- [#9](https://github.com/arkedge/c2a-tlmcmddb/pull/9): Update Rust crate csv to 1.2.2
- [#10](https://github.com/arkedge/c2a-tlmcmddb/pull/10): Update Rust crate clap to 4.3.4
- [#6](https://github.com/arkedge/c2a-tlmcmddb/pull/6): Update ghcr.io/sksat/cargo-chef-docker Docker tag to v1.70.0
- [#12](https://github.com/arkedge/c2a-tlmcmddb/pull/12): Update Rust crate clap to 4.3.8
- [#14](https://github.com/arkedge/c2a-tlmcmddb/pull/14): Update Rust crate serde to 1.0.183
- [#25](https://github.com/arkedge/c2a-tlmcmddb/pull/25): Explicitly assign release team (sksat)
- [#28](https://github.com/arkedge/c2a-tlmcmddb/pull/28): Merge workspace member package version into workspace.package.version
- [#19](https://github.com/arkedge/c2a-tlmcmddb/pull/19): Specify Rust version
- [#29](https://github.com/arkedge/c2a-tlmcmddb/pull/29): Introduce CI
- [#30](https://github.com/arkedge/c2a-tlmcmddb/pull/30): Update Swatinem/rust-cache action to v2.7.1
- [#31](https://github.com/arkedge/c2a-tlmcmddb/pull/31): Update actions/checkout action to v4.1.1
- [#17](https://github.com/arkedge/c2a-tlmcmddb/pull/17): Update Rust crate serde to 1.0.193
- [#18](https://github.com/arkedge/c2a-tlmcmddb/pull/18): Update Rust crate csv to 1.3.0
- [#13](https://github.com/arkedge/c2a-tlmcmddb/pull/13): Update Rust crate clap to 4.4.11


## 0.2.0 (2023-06-05)

### Added

- [#11](https://github.com/arkedge/c2a-tlmcmddb/pull/11): Add --component-name flag to bundle subcommand

### Internal

- [#1](https://github.com/arkedge/c2a-tlmcmddb/pull/1): Configure Renovate
- [#5](https://github.com/arkedge/c2a-tlmcmddb/pull/5): Update Rust crate csv to 1.2.1
- [#2](https://github.com/arkedge/c2a-tlmcmddb/pull/2): Update Rust crate serde to 1.0.160
- [#3](https://github.com/arkedge/c2a-tlmcmddb/pull/3): Update Rust crate clap to 4.2.4
- [#7](https://github.com/arkedge/c2a-tlmcmddb/pull/7): Update Rust crate clap to 4.3.0


## 0.1.0 (2023-04-14)

fist release

- `tlmcmddb`: a crate provides data model
- `tlmcmddb-csv`: a TlmCmd DB CSV parser
- `tlmcmddb-cli`: a CLI tool converts CSV to JSON
