use core::marker::PhantomData;
use kiss3d::{
	event::{Key, Modifiers, MouseButton},
	nalgebra::RealField,
};

/// Input keys/buttons and their modifiers.
#[derive(Debug, Clone)]
pub struct Input<N: Copy + RealField> {
	phantom_data: PhantomData<N>,
	first_key: Option<Key>,
	ortho_key: Option<Key>,
	reset_key: Option<Key>,
	orbit_button: Option<MouseButton>,
	orbit_modifiers: Option<Modifiers>,
	slide_button: Option<MouseButton>,
	slide_modifiers: Option<Modifiers>,
}

impl<N: Copy + RealField> Default for Input<N> {
	fn default() -> Self {
		Self {
			phantom_data: PhantomData,
			first_key: Some(Key::LShift),
			ortho_key: Some(Key::O),
			reset_key: Some(Key::Return),
			orbit_button: Some(MouseButton::Button1),
			orbit_modifiers: None,
			slide_button: Some(MouseButton::Button2),
			slide_modifiers: None,
		}
	}
}

impl<N: Copy + RealField> Input<N> {
	/// Key used to enable first person view as long as being pressed.
	#[must_use]
	pub fn first_key(&self) -> Option<Key> {
		self.first_key
	}
	/// Sets key used to enable first person view as long as being pressed.
	///
	/// Use `None` to disable key.
	pub fn rebind_first_key(&mut self, key: Option<Key>) {
		self.first_key = key;
	}
	/// Key used to switch between orthographic and perspective projection.
	#[must_use]
	pub fn ortho_key(&self) -> Option<Key> {
		self.ortho_key
	}
	/// Sets key used to switch between orthographic and perspective projection.
	///
	/// Use `None` to disable key.
	pub fn rebind_ortho_key(&mut self, key: Option<Key>) {
		self.ortho_key = key;
	}
	/// Key used to reset camera.
	#[must_use]
	pub fn reset_key(&self) -> Option<Key> {
		self.reset_key
	}
	/// Sets key used to reset camera.
	///
	/// Use `None` to disable key.
	pub fn rebind_reset_key(&mut self, key: Option<Key>) {
		self.reset_key = key;
	}
	/// Button used to orbit camera.
	#[must_use]
	pub fn orbit_button(&self) -> Option<MouseButton> {
		self.orbit_button
	}
	/// Sets button used to orbit camera.
	///
	/// Use `None` to disable button.
	pub fn rebind_orbit_button(&mut self, button: Option<MouseButton>) {
		self.orbit_button = button;
	}
	/// Modifiers that must be pressed for orbit to occur.
	#[must_use]
	pub fn orbit_modifiers(&self) -> Option<Modifiers> {
		self.orbit_modifiers
	}
	/// Sets modifiers that must be pressed for orbit to occur.
	///
	///   * If set to `None`, then pressing any modifier will not prevent orbit.
	///   * If different from `None`, orbit will occur only if the exact specified set of modifiers
	///     is pressed. In particular, if set to `Some(Modifiers::empty())`, orbit will occur only
	///     if no modifier is pressed.
	pub fn set_orbit_modifiers(&mut self, modifiers: Option<Modifiers>) {
		self.orbit_modifiers = modifiers;
	}
	/// Button used to slide camera.
	#[must_use]
	pub fn slide_button(&self) -> Option<MouseButton> {
		self.slide_button
	}
	/// Sets button used to slide camera.
	///
	/// Use `None` to disable button.
	pub fn rebind_slide_button(&mut self, button: Option<MouseButton>) {
		self.slide_button = button;
	}
	/// Modifiers that must be pressed for slide to occur.
	#[must_use]
	pub fn slide_modifiers(&self) -> Option<Modifiers> {
		self.slide_modifiers
	}
	/// Sets modifiers that must be pressed for slide to occur.
	///
	///   * If set to `None`, then pressing any modifier will not prevent slide.
	///   * If different from `None`, slide will occur only if the exact specified set of modifiers
	///     is pressed. In particular, if set to `Some(Modifiers::empty())`, slide will occur only
	///     if no modifier is pressed.
	pub fn set_slide_modifiers(&mut self, modifiers: Option<Modifiers>) {
		self.slide_modifiers = modifiers;
	}
}
