//! Virtual Trackball Camera Mode for Kiss3D
//!
//! Complements common [`trackball`] operation handlers with [`kiss3d`]-specific [`Input`] resulting
//! in a compound [`Trackball`] [`Camera`] mode implementation for the [`kiss3d`] graphics library.

#![allow(clippy::collapsible_else_if)]
#![no_std]

use kiss3d::{
	camera::Camera,
	event::{Action, TouchAction, WindowEvent},
	resource::ShaderUniform,
	window::Canvas,
};
use nalgebra::{Isometry3, Matrix4, Point2, Point3, UnitQuaternion, Vector3};
use trackball::{Clamp, First, Frame, Image, Orbit, Scale, Scene, Slide, Touch};

mod input;
pub use input::*;

pub use trackball::Fixed;

/// Trackball camera mode.
///
/// A trackball camera is a camera working similarly like a trackball device. The camera eye orbits
/// around its target, the trackball's center, and always looks at it. This implementation is split
/// into several members defined by the [`trackball`] crate categorizing all the methods to control
/// the different aspects of a camera.
///
/// # Camera Input
///
/// Following default inputs are defined which are customizable via [`Self::input`]:
///
/// Mouse                       | Touch                          | Action
/// --------------------------- | ------------------------------ | ---------------------------------
/// Left Button Press + Drag    | One-Finger + Drag              | Orbits around target.
/// ↳ but at trackball's border | Two-Finger + Roll              | Rolls about view direction.
/// Drag + Left Shift           | Any-Finger + Drag + Left Shift | First person view.
/// Right Button Press + Drag   | Two-Finger + Drag              | Slides trackball on focus plane.
/// Scroll In/Out               | Two-Finger + Pinch Out/In      | Scales distance zooming in/out.
/// Left Button Press + Release | Any-Finger + Release           | Slides to cursor/finger position.
///
/// Keyboard                    | Action
/// --------------------------- | ---------------------------------------------------------
/// O                           | Switches between orthographic and perspective projection.
/// Enter                       | Resets camera eye and target to [`Self::reset`].
///
/// # Camera Alignment
///
/// Realign camera via [`Self::frame`] and define user boundary conditions via [`Self::clamp`] like
/// minimum and maximum target distance from camera eye. Optionally, update the alignment to reset
/// to when pressing [`Input::reset_key()`] via [`Self::reset`].
///
/// # Camera Projection
///
/// Adjust camera projection via [`Self::scene`] like setting field of view or clip plane distances.
#[derive(Clone)]
pub struct Trackball {
	/// Input keys/buttons and their modifiers.
	pub input: Input<f32>,
	/// Clamp as user boundary conditions of [`Frame`].
	pub clamp: Clamp<f32>,
	/// Frame wrt camera eye and target.
	pub frame: Frame<f32>,
	/// Reset frame wrt camera eye and target.
	pub reset: Frame<f32>,
	/// Scene wrt enclosing viewing frustum.
	pub scene: Scene<f32>,

	image: Image<f32>,
	first: First<f32>,
	orbit: Orbit<f32>,
	scale: Scale<f32>,
	slide: Slide<f32>,
	touch: Touch<Option<u64>, f32>,
	mouse: Option<Point2<f64>>,
}

impl Trackball {
	/// Creates camera with eye position inclusive its roll attitude and target position.
	///
	/// Default viewing frustum has a fixed vertical field of view of π/4 with near and far clip
	/// planes at 1E-1 and 1E+6.
	///
	/// **Note:** Argument order differs from cameras in [`kiss3d::camera`].
	pub fn new(target: Point3<f32>, eye: &Point3<f32>, up: &Vector3<f32>) -> Trackball {
		let frame = Frame::look_at(target, eye, up);
		let reset = frame.clone();
		let scene = Scene::default();
		let image = Image::new(&frame, &scene, Point2::new(800.0, 600.0));
		Self {
			input: Default::default(),
			clamp: Default::default(),
			first: Default::default(),
			frame,
			reset,
			scene,
			image,
			orbit: Default::default(),
			scale: Default::default(),
			slide: Default::default(),
			touch: Default::default(),
			mouse: Default::default(),
		}
	}
	/// Like [`Self::new()`] but with custom viewing frustum.
	///
	/// For a fixed vertical field of view simply pass an [`f32`] angle in radians as `fov`,
	/// otherwise see [`Fixed`] and [`Scene::set_fov()`].
	///
	/// **Note:** Argument order differs from cameras in [`kiss3d::camera`].
	pub fn new_with_frustum(
		target: Point3<f32>,
		eye: &Point3<f32>,
		up: &Vector3<f32>,
		fov: impl Into<Fixed<f32>>,
		znear: f32,
		zfar: f32,
	) -> Trackball {
		let mut trackball = Self::new(target, eye, up);
		trackball.scene.set_fov(fov);
		trackball.scene.set_clip_planes(znear, zfar);
		trackball
	}
}

impl Camera for Trackball {
	fn clip_planes(&self) -> (f32, f32) {
		self.scene.clip_planes(self.frame.distance())
	}
	fn view_transform(&self) -> Isometry3<f32> {
		*self.image.view_isometry()
	}
	fn eye(&self) -> Point3<f32> {
		self.frame.eye()
	}
	fn handle_event(&mut self, canvas: &Canvas, event: &WindowEvent) {
		match *event {
			WindowEvent::Touch(id, x, y, action, _modifiers) => {
				let pos = Point2::new(x as f32, y as f32);
				match action {
					TouchAction::Start | TouchAction::Move => {
						if action == TouchAction::Start {
							self.slide.discard();
						}
						if let Some((num, pos, rot, rat)) = self.touch.compute(Some(id), pos, 0) {
							if self.first.enabled() {
								if let Some(vec) = self.slide.compute(pos) {
									if let Some((pitch, yaw, yaw_axis)) =
										self.first.compute(&vec, self.image.max())
									{
										self.frame.look_around(pitch, yaw, yaw_axis);
									}
								}
							} else {
								if num == 1 {
									if let Some(rot) = self.orbit.compute(&pos, self.image.max()) {
										self.frame.local_orbit(&rot);
									}
								} else {
									if let Some(vec) = self.slide.compute(pos) {
										self.frame.local_slide(&self.image.project_vec(&vec));
									}
									if num == 2 {
										let pos = self.image.project_pos(&pos);
										let rot = UnitQuaternion::from_axis_angle(
											&self.frame.local_roll_axis(),
											rot,
										);
										self.frame.local_orbit_around(&rot, &pos);
										self.frame.local_scale_around(rat, &pos);
									}
								}
							}
						}
					}
					TouchAction::End | TouchAction::Cancel => {
						if let Some((_num, pos)) = self.touch.discard(Some(id)) {
							self.frame.local_slide(&self.image.project_pos(&pos).coords);
						}
						self.orbit.discard();
						self.slide.discard();
					}
				}
			}
			WindowEvent::MouseButton(button, action, _modifiers) if !self.first.enabled() => {
				if Some(button) == self.input.orbit_button() {
					if action == Action::Press {
						self.touch.compute(None, *self.image.pos(), 0);
					} else {
						self.orbit.discard();
						if let Some((_num, pos)) = self.touch.discard(None) {
							self.frame.local_slide(&self.image.project_pos(&pos).coords);
						}
					}
				}
				if Some(button) == self.input.slide_button() {
					if action == Action::Press {
						self.slide.compute(*self.image.pos());
					} else {
						self.slide.discard();
					}
				}
			}
			WindowEvent::CursorPos(x, y, modifiers) => {
				let pos = Point2::new(x, y);
				let is_eq = |old| old == pos || old == Point2::new(pos.x.floor(), pos.y.floor());
				if self.mouse.replace(pos).map_or(true, is_eq) {
					return;
				}
				let (pos, max) = (pos.cast(), *self.image.max());
				if self.first.enabled() {
					if self.touch.fingers() == 0 {
						if let Some(vec) = self.slide.compute(pos) {
							canvas.hide_cursor(true);
							canvas.set_cursor_grab(true);
							if let Some((pitch, yaw, yaw_axis)) = self.first.compute(&vec, &max) {
								self.frame.look_around(pitch, yaw, yaw_axis);
							}
						}
						if pos.y <= 0.0 {
							canvas.set_cursor_position(x, max.y as f64 - 2.0);
							self.slide.discard();
						}
						if pos.x <= 0.0 {
							canvas.set_cursor_position(max.x as f64 - 2.0, y);
							self.slide.discard();
						}
						if pos.x >= max.x - 1.0 {
							canvas.set_cursor_position(1.0, y);
							self.slide.discard();
						}
						if pos.y >= max.y - 1.0 {
							canvas.set_cursor_position(x, 1.0);
							self.slide.discard();
						}
					}
				} else {
					self.image.set_pos(pos);
					let orbit = self.input.orbit_button().map_or(false, |button| {
						canvas.get_mouse_button(button) == Action::Press
							&& self
								.input
								.orbit_modifiers()
								.map(|m| m == modifiers)
								.unwrap_or(true)
					});
					let slide = self.input.slide_button().map_or(false, |button| {
						canvas.get_mouse_button(button) == Action::Press
							&& self
								.input
								.slide_modifiers()
								.map(|m| m == modifiers)
								.unwrap_or(true)
					});
					if orbit && slide {
						self.orbit.discard();
						self.slide.discard();
					}
					if orbit {
						if let Some(pos) = self.touch.compute(None, pos, 0).map(|val| val.1) {
							if let Some(rot) = self.orbit.compute(&pos, &max) {
								self.frame.local_orbit(&rot);
							}
						}
					}
					if slide {
						if let Some(vec) = self.slide.compute(pos) {
							self.frame.local_slide(&self.image.project_vec(&vec));
						}
					}
				}
			}
			WindowEvent::Scroll(_, val, _) => {
				self.frame.local_scale_around(
					self.scale.compute(val as f32),
					&self.image.project_pos(self.image.pos()),
				);
			}
			WindowEvent::Key(key, action, _modifiers) if Some(key) == self.input.first_key() => {
				let mid = self.image.max() * 0.5;
				if action == Action::Press {
					if !self.first.enabled() {
						self.first.capture(self.frame.yaw_axis());
						self.image.set_pos(mid);
					}
				} else {
					self.slide.discard();
					self.first.discard();
					if self.touch.fingers() == 0 {
						canvas.set_cursor_position(mid.x as f64, mid.y as f64);
						canvas.hide_cursor(false);
						canvas.set_cursor_grab(false);
					}
				}
			}
			WindowEvent::Key(key, Action::Press, _modifiers)
				if Some(key) == self.input.ortho_key() =>
			{
				self.scene.set_ortho(!self.scene.ortho());
			}
			WindowEvent::Key(key, Action::Press, _modifiers)
				if Some(key) == self.input.reset_key() =>
			{
				self.frame = self.reset.clone();
			}
			WindowEvent::FramebufferSize(w, h) => {
				self.image.set_max(Point2::new(w, h).cast());
			}
			_ => {}
		}
	}
	#[inline]
	fn upload(
		&self,
		_: usize,
		proj: &mut ShaderUniform<Matrix4<f32>>,
		view: &mut ShaderUniform<Matrix4<f32>>,
	) {
		proj.upload(self.image.projection());
		view.upload(self.image.view());
	}
	fn transformation(&self) -> Matrix4<f32> {
		*self.image.transformation()
	}
	fn inverse_transformation(&self) -> Matrix4<f32> {
		*self.image.inverse_transformation()
	}
	fn update(&mut self, _: &Canvas) {
		self.frame = self.clamp.compute(self.frame.clone(), &self.scene);
		self.image.compute(self.frame.clone(), self.scene.clone());
	}
}
