# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
