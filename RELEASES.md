# Version 0.6.1 (2023-09-02)

  * Deprecate in favor of [`bevy_trackball`].

    [`bevy_trackball`]: https://qu1x.github.io/bevy_trackball

# Version 0.6.0 (2022-03-22)

  * Re-export `kiss3d` and `trackball` moving `Fixed` to `trackball::Fixed`.

# Version 0.5.0 (2022-03-19)

  * Bump `kiss3d`, `trackball`, and `nalgebra` to latest versions.
  * Adhere to pedantic Clippy lints.

# Version 0.4.0 (2021-11-12)

  * Bump `kiss3d`, `trackball`, and `nalgebra` to latest versions.
  * Use latest edition.

# Version 0.3.1 (2021-08-24)

  * Prepare for latest `nalgebra`.
  * Make crate `no_std`.

# Version 0.3.0 (2021-04-28)

  * Add first person view.
  * Use move semantics whenever otherwise cloning borrowed method arguments.
  * Reorder arguments of `Trackball::new()` and `Trackball::new_with_frustum()`.

# Version 0.2.0 (2021-04-13)

  * Use `Fixed` quantity wrt field of view.
  * Use better supported Unicode arrow symbol.
  * Update dependencies.

# Version 0.1.1 (2021-04-08)

  * Switch to [BSD-2-Clause-Patent](LICENSES/BSD-2-Clause-Patent.md).

# Version 0.1.0 (2021-04-03)

  * Add trackball implementation.
