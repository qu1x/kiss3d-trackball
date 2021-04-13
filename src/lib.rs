//! Virtual Trackball Camera Mode for Kiss3D
//!
//! Complements common [`trackball`] operation handlers with [`kiss3d`]-specific [`Input`] resulting
//! in a compound [`Trackball`] [`Camera`] mode implementation for the [`kiss3d`] graphics library.

use kiss3d::{
	camera::Camera,
	event::{Action, TouchAction, WindowEvent},
	resource::ShaderUniform,
	window::Canvas,
};
use nalgebra::{Isometry3, Matrix4, Point2, Point3, UnitQuaternion, Vector3};
use trackball::{Clamp, Frame, Image, Orbit, Scale, Scene, Slide, Touch};

mod input;
pub use input::*;

pub use trackball::Fixed;

/// Trackball camera mode.
///
/// A trackball camera is a camera working similarly like a trackball device. The camera eye is
/// orbited around its target, the trackball's center, and always looking at it. This implementation
/// is split into several members defined by the [`trackball`] crate categorizing all the methods to
/// control the different aspects of a camera.
///
/// # Camera Controls
///
/// Following default controls are defined which are customizable via [`Self::input`]:
///
/// Mouse                       | Touch                     | Action
/// --------------------------- | ------------------------- | --------------------------------------
/// Left Button Press + Drag    | One-Finger + Drag         | Orbits eye around target.
/// ⮱ but at trackball's border | Two-Finger + Roll         | Purely rolls eye about view direction.
/// Right Button Press + Drag   | Two-Finger + Drag         | Slides trackball along focus plane.
/// Scroll In/Out               | Two-Finger + Pinch Out/In | Scales distance zooming in/out.
/// Left Button Press + Release | Any-Finger + Release      | Slides to cursor/finger position.
///
/// Key                         | Action
/// --------------------------- | -------------------------------------------------------
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
	orbit: Orbit<f32>,
	scale: Scale<f32>,
	slide: Slide<f32>,
	touch: Touch<Option<u64>, f32>,
}

impl Trackball {
	/// Creates camera with eye position inclusive its roll attitude and target position.
	///
	/// Default frustum has field of view of π/4 with near and far clip planes at 1E-1 and 1E+6.
	pub fn new(eye: &Point3<f32>, at: &Point3<f32>, up: &Vector3<f32>) -> Trackball {
		let frame = Frame::look_at(eye, at, up);
		let reset = frame.clone();
		let scene = Scene::default();
		let image = Image::new(&frame, &scene, &Point2::new(800.0, 600.0));
		Self {
			input: Default::default(),
			clamp: Default::default(),
			frame,
			reset,
			scene,
			image,
			orbit: Default::default(),
			scale: Default::default(),
			slide: Default::default(),
			touch: Default::default(),
		}
	}
	/// Like [`Self::new()`] but with custom frustum.
	pub fn new_with_frustum(
		fov: impl Into<Fixed<f32>>,
		znear: f32,
		zfar: f32,
		eye: &Point3<f32>,
		at: &Point3<f32>,
		up: &Vector3<f32>,
	) -> Trackball {
		let mut trackball = Self::new(eye, at, up);
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
		self.image.view_isometry().clone()
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
						if let Some((num, pos, rot, rat)) = self.touch.compute(Some(id), &pos, 0) {
							if num == 1 {
								if let Some(rot) = self.orbit.compute(&pos, self.image.max()) {
									self.frame.local_orbit(&rot);
								}
							} else {
								if let Some(vec) = self.slide.compute(&pos) {
									self.frame.local_slide(&self.image.project_vec(&vec));
								}
								let pos = self.image.project_pos(&pos);
								let rot = UnitQuaternion::from_axis_angle(
									&self.frame.local_roll_axis(),
									rot,
								);
								self.frame.local_orbit_at(&rot, &pos);
								self.frame.local_scale_at(rat, &pos);
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
			WindowEvent::MouseButton(button, action, _modifiers) => {
				if Some(button) == self.input.orbit_button() {
					if action == Action::Press {
						self.touch.compute(None, self.image.pos(), 0);
					} else {
						self.orbit.discard();
						if let Some((_num, pos)) = self.touch.discard(None) {
							self.frame.local_slide(&self.image.project_pos(&pos).coords);
						}
					}
				}
				if Some(button) == self.input.slide_button() {
					if action == Action::Press {
						let pos = self.image.pos().clone();
						if let Some(vec) = self.slide.compute(&pos) {
							self.frame.local_slide(&self.image.project_vec(&vec));
						}
					} else {
						self.slide.discard();
					}
				}
			}
			WindowEvent::CursorPos(x, y, modifiers) => {
				let pos = Point2::new(x as f32, y as f32);
				self.image.set_pos(&pos);
				if let Some(orbit_button) = self.input.orbit_button() {
					if canvas.get_mouse_button(orbit_button) == Action::Press
						&& self
							.input
							.orbit_modifiers()
							.map(|m| m == modifiers)
							.unwrap_or(true)
					{
						if let Some((_num, _pos, _rot, _rat)) = self.touch.compute(None, &pos, 0) {
							if let Some(rot) = self.orbit.compute(&pos, self.image.max()) {
								self.frame.local_orbit(&rot);
							}
						}
					}
				}
				if let Some(slide_button) = self.input.slide_button() {
					if canvas.get_mouse_button(slide_button) == Action::Press
						&& self
							.input
							.slide_modifiers()
							.map(|m| m == modifiers)
							.unwrap_or(true)
					{
						if let Some(vec) = self.slide.compute(&pos) {
							self.frame.local_slide(&self.image.project_vec(&vec));
						}
					}
				}
			}
			WindowEvent::Scroll(_, val, _) => {
				self.frame.local_scale_at(
					self.scale.compute(val as f32),
					&self.image.project_pos(self.image.pos()),
				);
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
				self.image.set_max(&Point2::new(w as f32, h as f32));
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
		self.image.transformation().clone()
	}
	fn inverse_transformation(&self) -> Matrix4<f32> {
		self.image.inverse_transformation().clone()
	}
	fn update(&mut self, _: &Canvas) {
		self.frame = self.clamp.compute(&self.frame, &self.scene);
		self.image.compute(&self.frame, &self.scene);
	}
}
