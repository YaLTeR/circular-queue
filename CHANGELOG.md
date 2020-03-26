# Changelog

## [Unreleased]

### Changed
- `CircularQueue::with_capacity()` now accepts zero capacity without panicking.

## [0.2.3] - 2020-01-09

### Added
- `CircularQueue::asc_iter()` and `asc_iter_mut()` for iterating over the queue
  items in oldest-to-newest order.
- Marked `CircularQueue::push()` as `#[inline]`.

## [0.2.2] - 2019-09-01

### Added
- `PartialEq` and `Eq` implementations for `CircularQueue`.

## [0.2.1] - 2019-08-02

### Added
- `#![no_std]` support on Rust >= `1.36.0`.

## [0.2.0] - 2017-07-24

### Added
- `CircularQueue::is_empty()`.
- Zero-sized type support.

### Changed
- Renamed `CircularQueue::new()` to `with_capacity()`.

## [0.1.2] - 2017-07-21

### Added
- `CircularQueue::iter_mut()`.

[Unreleased]: https://github.com/YaLTeR/circular-queue/compare/v0.2.3...HEAD
[0.2.3]: https://github.com/YaLTeR/circular-queue/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/YaLTeR/circular-queue/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/YaLTeR/circular-queue/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/YaLTeR/circular-queue/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/YaLTeR/circular-queue/compare/v0.1.1...v0.1.2
