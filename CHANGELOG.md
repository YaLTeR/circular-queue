# Changelog

## [Unreleased]

## [0.2.7] - 2025-03-14

### Added
- `CircularQueue::into_vec()` for converting into a `Vec` without extra allocations.
- A corresponding `From` impl.

### Fixed
- Warnings on newer Rust versions.

## [0.2.6] - 2020-07-27

### Added
- `CircularQueue::is_full()` for checking whether the queue is completely filled.

### Changed
- `CircularQueue::push()` now returns the element it overwrites, if any.

## [0.2.5] - 2020-06-21

### Added
- Serde support under the `serde_support` feature.

## [0.2.4] - 2020-03-26

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

[Unreleased]: https://github.com/YaLTeR/circular-queue/compare/v0.2.7...HEAD
[0.2.7]: https://github.com/YaLTeR/circular-queue/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/YaLTeR/circular-queue/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/YaLTeR/circular-queue/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/YaLTeR/circular-queue/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/YaLTeR/circular-queue/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/YaLTeR/circular-queue/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/YaLTeR/circular-queue/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/YaLTeR/circular-queue/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/YaLTeR/circular-queue/compare/v0.1.1...v0.1.2
