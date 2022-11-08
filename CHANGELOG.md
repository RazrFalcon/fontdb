# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.10.0] - 2022-11-08
### Added
- `no_std` support. [@jackpot51](https://github.com/jackpot51)

## [0.9.3] - 2022-10-26
### Added
- `Database::family_name` is public now.

## [0.9.2] - 2022-10-22
### Added
- `Database::push_face_info`
- `ID::dummy`

### Fixed
- Expand home path `~` prefix during fontconfig paths resolving.
  [@snoyer](https://github.com/snoyer)

## [0.9.1] - 2022-02-21
### Changed
- Reduce binary size by 10% using less generic code.
- Simplify Database::query implementation.

## [0.9.0] - 2022-02-20
### Added
- Way faster fonts scanning by using a more low-level `ttf-parser` API
  which allows us to parse only required TrueType tables.
  On my hardware, `load_system_fonts()` loaded 898 fonts in 9ms instead of 11ms in the release mode
  and in 35ms instead of 52ms in debug.
  Currently, we're parsing only `name`, `OS/2` and `post` tables.

## [0.8.0] - 2022-02-12
### Added
- Load user fonts on Windows.
- `fontconfig` feature to allow retrieving font dirs from the fontconfig config file
  instead of using hardcoded paths. Linux-only. [@Riey](https://github.com/Riey)

## [0.7.0] - 2021-10-04
### Changed
- The `Source` enum has a new variant `SharedFile`, used for unsafe persistent
  memory mappings.
- `FaceInfo` stores `Source` directly now, not anymore in an `Arc`. Instead `Source::Binary`
  now stores an `Arc` of the data.

## [0.6.2] - 2021-09-04
### Fixed
- Fix compilation without the `fs` feature.

## [0.6.1] - 2021-09-04
### Changed
- Split the `fs` build feature into `fs` and `memmap`. [@neinseg](https://github.com/neinseg)

## [0.6.0] - 2021-08-21
### Added
- Search in `$HOME/.fonts` on Linux. [@Linus789](https://github.com/Linus789)

### Changed
- Generic font families are preset by default instead of being set to `None`.

## [0.5.4] - 2021-05-25
### Added
- Implement `Eq`, `Hash` for `Query`, `Family`, `Weight` and `Style`.
  [@dhardy](https://github.com/dhardy)

### Changed
- Update `ttf-parser`

## [0.5.3] - 2021-05-19
### Changed
- Update `ttf-parser`

## [0.5.2] - 2021-05-19
### Changed
- Update `memmap2`
- Add additional search dir for macOS.

## [0.5.1] - 2020-12-20
### Fixed
- Compilation on Windows.

## [0.5.0] - 2020-12-20
### Added
- `FaceInfo::post_script_name`
- `FaceInfo::monospaced`
- `Database::load_system_fonts`

## [0.4.0] - 2020-12-06
### Changed
- Use a simple `u32` for ID instead of UUID.

## [0.3.0] - 2020-12-05
### Changed
- `ttf-parser` updated.

## [0.2.0] - 2020-07-21
### Changed
- `ttf-parser` updated.

### Fixed
- Stretch processing. `ttf-parser` was incorrectly parsing this property.

[Unreleased]: https://github.com/RazrFalcon/fontdb/compare/v0.10.0...HEAD
[0.10.0]: https://github.com/RazrFalcon/fontdb/compare/v0.9.3...v0.10.0
[0.9.3]: https://github.com/RazrFalcon/fontdb/compare/v0.9.2...v0.9.3
[0.9.2]: https://github.com/RazrFalcon/fontdb/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/RazrFalcon/fontdb/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/RazrFalcon/fontdb/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/RazrFalcon/fontdb/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/RazrFalcon/fontdb/compare/v0.6.2...v0.7.0
[0.6.2]: https://github.com/RazrFalcon/fontdb/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/RazrFalcon/fontdb/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/RazrFalcon/fontdb/compare/v0.5.4...v0.6.0
[0.5.4]: https://github.com/RazrFalcon/fontdb/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/RazrFalcon/fontdb/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/RazrFalcon/fontdb/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/RazrFalcon/fontdb/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/RazrFalcon/fontdb/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/RazrFalcon/fontdb/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/RazrFalcon/fontdb/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/RazrFalcon/fontdb/compare/v0.1.0...v0.2.0
