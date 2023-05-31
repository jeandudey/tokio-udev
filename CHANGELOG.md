<!--
SPDX-FileCopyrightText: © 2020 Jean-Pierre De Jesus DIAZ <me@jeandudey.tech>
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# [unreleased]

# 0.9.0

- tokio-udev: update udev to 0.7.
- tokio-udev: set MSRV as 1.56.
- tokio-udev: fix bug where some events would not be retrieved (#17).
- tree-wide: use REUSE to identify copyright in each file.

# 0.8.0

- tokio-udev: by mistake the version released contains the same code as 0.7.0, no version
was tagged.

# 0.7.0

- tokio-udev: remove dependency to `mio-udev` and use the provided
mio implementation of the `udev` crate.
- tokio-udev: fix Send/Sync soundness (see #13). As a result, the Send/Sync implementations
are removed to avoid double-free/undefined behaviour.
- tokio-udev: rename `MonitorSocket` to `AsyncMonitorSocket` to keep the existing type from
udev.

# 0.6.0

- tokio-udev: use tokio `0.1`.

# 0.5.0

- mio-udev,tokio-udev: update udev to 0.5 [#12](https://github.com/jeandudey/tokio-udev/pull/12).

# 0.4.0

- tokio-udev: Update tokio to `>=0.3.2` [#9](https://github.com/jeandudey/tokio-udev/pull/9).
- tokio-udev: Update mio to `0.7` [#9](https://github.com/jeandudey/tokio-udev/pull/9).
- tokio-udev: Update mio-udev to `0.4` [#11](https://github.com/jeandudey/tokio-udev/pull/11).
- mio-udev: Update mio to `0.7`.

# 0.3.0

- .github: Add Rust actions ([#5](https://github.com/jeandudey/tokio-udev/pull/5))
- {mio,tokio}-udev: format code with rustfmt ([#4](https://github.com/jeandudey/tokio-udev/pull/4))
  - mio-udev: simplify if-expression
- Add missing documention ([#7](https://github.com/jeandudey/tokio-udev/pull/7))
  - CHANGELOG: add basic changelog
  - README: add README.md
  - {mio,tokio}-udev: update crate descriptions
  - {mio,tokio}-udev: apply copyright
  - Add LICENSE-MIT and LICENSE-APACHE
- mio-udev: update udev to 0.4 [#8](https://github.com/jeandudey/tokio-udev/pull/8)

# 0.2.0

- Update to tokio 0.2 ([#3](https://github.com/jeandudey/tokio-udev/pull/3))
 - Expose udev::Enumerator for public use
 - Migrate to Rust 2018
 - Update to tokio 0.2

# 0.1.0

- Initial Release
