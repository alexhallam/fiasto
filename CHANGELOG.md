## [0.2.1] - 2025-09-04

just to trigger a new release

## [0.2.0] - 2025-09-04

### Added

- **Advanced Random Effects Syntax**: Complete brms-style random effects parsing including:
  - Random intercepts and slopes: `(1 | group)`, `(0 + x | group)`, `(-1 + x | group)`
  - Correlation types: `|` (correlated), `||` (uncorrelated), `|ID|` (cross-parameter)
  - Enhanced grouping with `gr()` function supporting `cor`, `id`, `by`, `cov`, `dist` arguments... kind of still WIP
  - Multi-membership structures: `mm()`, `mmc()`... WIP
  - Category-specific effects: `cs()`... WIP
  - Hierarchical/nested structures: `group1/group2` ... WIP
  - Interaction of grouping factors: `group1:group2` ... WIP
  - Suppression of random intercepts: `0 +` or `-1 +`
- **Variable-Centric Metadata Structure**: Complete redesign of output format:
  - Variables as first-class citizens with comprehensive attributes
  - Roles: Response, FixedEffect, GroupingVariable, RandomEffect
  - Transformations with generated column tracking
  - Interactions with proper naming
  - Random effects information per variable
- **Enhanced Function Support**: Added support for `log()` function and improved function parsing
- **Interaction Support**: Fixed effects interactions with `x:z` syntax
- **Family Information**: Always included in metadata output
- **Generated Columns Tracking**: Every variable tracks its generated columns
- **ID Ordering**: Response variable always gets ID 1, others start from ID 2
- **All Generated Columns Array**: Complete list of all generated columns ordered by ID
- **Comprehensive Documentation**: Added detailed `gr()` function documentation .. WIP

### Changed

- **Breaking Change**: Complete metadata structure redesign from effect-centric to variable-centric
- **Breaking Change**: New JSON output format with variables as primary entities
- **Breaking Change**: Version bump to 0.2.0 due to significant API changes

### Fixed

- Fixed grouping factor inclusion in random effects columns
- Fixed token ordering in lexer for proper keyword recognition
- Fixed interaction parsing in complex formulas
- Fixed hardcoded interaction naming

## [0.1.6] - 2025-09-03

### Added

- added more detailed doc output

## [0.1.5] - 2025-09-03

### Added

- spelling

## [0.1.4] - 2025-09-03

### Added

- new logo

## [0.1.3] - 2025-09-03

### Removed

- some toml keywords. only 5 allowed.

## [0.1.2] - 2025-09-03

### Added

- patch for release workflow

## [0.1.1] - 2025-09-03

### Added

- patch for release workflow

## [0.1.0] - 2025-09-03

### Added

- Initial release