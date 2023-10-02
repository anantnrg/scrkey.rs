use gtk::{
	glib,
	prelude::*,
	Application,
	ApplicationWindow,
};

pub fn create_window(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.title("ScrKey.rs")
		.default_width(200)
		.default_height(100)
		.modal(true)
		.build();

	window.present();
}
