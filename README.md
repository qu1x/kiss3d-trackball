# kiss3d-trackball

Coherent Virtual Trackball Camera Mode for Kiss 3D

[![Build][]](https://github.com/qu1x/kiss3d-trackball/actions/workflows/build.yml)
[![Documentation][]](https://docs.rs/kiss3d-trackball)
[![Downloads][]](https://crates.io/crates/kiss3d-trackball)
[![Version][]](https://crates.io/crates/kiss3d-trackball)
[![Rust][]](https://www.rust-lang.org)
[![License][]](https://spdx.org/licenses/BSD-2-Clause-Patent.html)

[Build]: https://github.com/qu1x/kiss3d-trackball/actions/workflows/build.yml/badge.svg
[Documentation]: https://docs.rs/kiss3d-trackball/badge.svg
[Downloads]: https://img.shields.io/crates/d/kiss3d-trackball.svg
[Version]: https://img.shields.io/crates/v/kiss3d-trackball.svg
[Rust]: https://img.shields.io/badge/rust-stable-brightgreen.svg
[License]: https://img.shields.io/crates/l/kiss3d-trackball.svg

Complements the [trackball library] with Kiss3D-specific [`Input`] resulting in a compound
[`Trackball`] camera mode implementation for the [Kiss3D graphics library].

## Coherence

This is an alternative trackball technique using exponential map and parallel transport to
preserve distances and angles for inducing coherent and intuitive trackball rotations. For
instance, displacements on straight radial lines through the screen's center are carried to arcs
of the same length on great circles of the trackball (e.g., dragging the mouse along an eights
of the trackball's circumference rolls the camera by 360/8=45 degrees, dragging the mouse from
the screen's center to its further edge *linearly* rotates the camera by 1 [radian], where the
trackball's diameter is the maximum of the screen's width and height). This is in contrast to
state-of-the-art techniques using orthogonal projection which distorts radial distances further
away from the screen's center (e.g., the rotation accelerates towards the edge).[^1]

[^1]: G. Stantchev, “Virtual Trackball Modeling and the Exponential Map”, [S2CID 44199608 (2004)
](https://api.semanticscholar.org/CorpusID:44199608), [Archived PDF
](https://web.archive.org/web/2/http://www.math.umd.edu:80/~gogo/Papers/trackballExp.pdf)

[radian]: https://en.wikipedia.org/wiki/Radian

See the [release history](RELEASES.md) to keep track of the development.

[trackball library]: https://github.com/qu1x/trackball
[Kiss3D graphics library]: https://github.com/sebcrozet/kiss3d

[`Input`]: https://docs.rs/kiss3d-trackball/latest/kiss3d_trackball/struct.Input.html
[`Trackball`]: https://docs.rs/kiss3d-trackball/latest/kiss3d_trackball/struct.Trackball.html

## License

The works are [licensed](LICENSES/BSD-2-Clause-Patent.md) under the [`BSD-2-Clause-Patent`].

This license is designed to provide:

  * a simple permissive license,
  * that is compatible with the [`GPL-2.0-or-later`], and
  * which also has an express patent grant included, but
  * unlike the [`Apache-2.0`] without patent retaliation.

[`BSD-2-Clause-Patent`]: https://spdx.org/licenses/BSD-2-Clause-Patent.html
[`GPL-2.0-or-later`]: https://spdx.org/licenses/GPL-2.0-or-later.html
[`Apache-2.0`]: https://spdx.org/licenses/Apache-2.0.html

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the works by you shall be licensed as above, without any additional terms or conditions.
