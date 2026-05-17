# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.2](https://github.com/segfault-merchant/git-stratum/compare/v0.4.1...v0.4.2) - 2026-05-17

### Added

- define method to calculate number of deletions in a modified file

## [0.4.1](https://github.com/segfault-merchant/git-stratum/compare/v0.4.0...v0.4.1) - 2026-05-17

### Added

- define insertions method and test

## [0.4.0](https://github.com/segfault-merchant/git-stratum/compare/v0.3.8...v0.4.0) - 2026-05-16

### Added

- test mfile function and update test fixture
- define filename method, resolving the filename according to old and new path

## [0.3.8](https://github.com/segfault-merchant/git-stratum/compare/v0.3.7...v0.3.8) - 2026-05-16

### Added

- define a method to return the projects name

## [0.3.7](https://github.com/segfault-merchant/git-stratum/compare/v0.3.6...v0.3.7) - 2026-05-16

### Added

- define method to return the project path

## [0.3.6](https://github.com/segfault-merchant/git-stratum/compare/v0.3.5...v0.3.6) - 2026-05-16

### Added

- benchmark from_str
- define offset method

### Fixed

- capture the email string properly by slicing on index

### Other

- Merge branch 'main' into 41-actor-datetz

## [0.3.5](https://github.com/segfault-merchant/git-stratum/compare/v0.3.4...v0.3.5) - 2026-05-16

### Fixed

- update links
- remove double title

### Other

- add badges to readme

## [0.3.4](https://github.com/segfault-merchant/git-stratum/compare/v0.3.3...v0.3.4) - 2026-05-14

### Added

- basic benchmarks to enable performance regression checks

### Other

- replace String with str ref in Actor return types

## [0.3.3](https://github.com/segfault-merchant/git-stratum/compare/v0.3.2...v0.3.3) - 2026-05-13

### Other

- remove unused dev deps

## [0.3.2](https://github.com/segfault-merchant/git-stratum/compare/v0.3.1...v0.3.2) - 2026-05-09

### Added

- define method to check if the current commit is reachable from main
- Enable error comparison with PEq

### Other

- comment what commit is used in test

## [0.3.1](https://github.com/segfault-merchant/git-stratum/compare/v0.3.0...v0.3.1) - 2026-05-07

### Added

- Define methods to return all, local, and remote branches

### Other

- Test the new branch methods

## [0.3.0](https://github.com/segfault-merchant/git-stratum/compare/v0.2.4...v0.3.0) - 2026-05-06

### Added

- Define co_authors method
- Implement FromStr on Actor
- Add regex as dependency, include Regex error in stratum::Error

## [0.2.4](https://github.com/segfault-merchant/git-stratum/compare/v0.2.3...v0.2.4) - 2026-05-04

### Other

- Update issue templates

## [0.2.3](https://github.com/segfault-merchant/git-stratum/compare/v0.2.2...v0.2.3) - 2026-05-03

### Added

- Define test helper function to instantiate repo from git2::Repo
- Test new modified file struct
- define a commits modified files in terms of the delta and patches

### Fixed

- Ensure commit testing adheres to new test repo return
- unit test repo generation should return git2::Repo to enable more explciit testing

### Other

- import git2 objects to simplify content
- Prefer use of delta for delta-related methods as it's cheaper than patches
- release v0.2.2
- update README to match testing
- *(test)* Unitise test repo generation to ensure readability

## [0.2.2](https://github.com/segfault-merchant/git-stratum/compare/v0.2.1...v0.2.2) - 2026-05-01

### Other

- update README to match testing
- *(test)* Unitise test repo generation to ensure readability

## [0.2.1](https://github.com/segfault-merchant/git-stratum/compare/v0.2.0...v0.2.1) - 2026-04-28

### Added

- test new semantic release method

### Fixed

- update token name to match secrets

### Other

- to use release-plz quick start config
