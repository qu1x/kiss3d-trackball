use kiss3d_trackball::{
	Trackball,
	kiss3d::{
		light::Light,
		nalgebra::{Point3, Vector3},
		window::Window,
	},
};

#[kiss3d::main]
async fn main() {
	let target = Point3::origin();
	let eye = Point3::new(2.0, 2.0, 2.0);
	let up = Vector3::y_axis();
	let mut trackball = Trackball::new(target, &eye, &up);
	let mut window = Window::new("Coherent Virtual Trackball Camera Mode for Kiss 3D");
	window.set_light(Light::StickToCamera);
	while !window.should_close() {
		window.draw_line(
			&Point3::origin(),
			&Point3::new(1.0, 0.0, 0.0),
			&Point3::new(1.0, 0.0, 0.0),
		);
		window.draw_line(
			&Point3::origin(),
			&Point3::new(0.0, 1.0, 0.0),
			&Point3::new(0.0, 1.0, 0.0),
		);
		window.draw_line(
			&Point3::origin(),
			&Point3::new(0.0, 0.0, 1.0),
			&Point3::new(0.0, 0.0, 1.0),
		);
		window.render_with_camera(&mut trackball).await;
	}
}
