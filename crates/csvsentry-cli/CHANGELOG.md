# Changelog

All notable changes to the `csvsentry` CLI are documented here.

## [Unreleased]

### Added
-

### Changed
-

### Fixed
- 

## [0.3.0] - 2026-11-25

### Added
- Exploration tests covering type inference, required/optional columns, sampling, delimiters, and error cases.
- Settings generation tests (with and without exploration).
- `has_values` flag on `ExplorationResult` to distinguish empty columns.
- CLI `Generate` now uses exploration when `--explore-types` is set.

### Changed
- Type inference fallback: columns with no non-empty sampled values now default to `Text(None)`.
- Type precedence follows `ColumnType` variant order (documented in code); `Text(None)` is skipped for non-empty columns.
- All-empty columns yield an empty `possible_types` list instead of all variants.

### Fixed
- (keep existing) path resolver to match format

## [0.2.0] - 2026-11-23

- Added option to output verbose results instead of it being the default
- Added RowIndex Enum - allows for defining the "index type" - with or without header and with a 1-Based indexing for later use in Excel
- Added sample read - will later be expanded to exploration

## [0.1.1] - 2026-11-22

### Added

- `csvsentry update` self-update command.
- JSONL output option.

### Fixed

- Delimiter serialization in generated settings.

## [0.1.0] - 2026-11-20

- Initial CLI release.
