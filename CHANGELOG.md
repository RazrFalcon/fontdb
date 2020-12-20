# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
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

[Unreleased]: https://github.com/RazrFalcon/fontdb/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/RazrFalcon/fontdb/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/RazrFalcon/fontdb/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/RazrFalcon/fontdb/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/RazrFalcon/fontdb/compare/v0.1.0...v0.2.0
