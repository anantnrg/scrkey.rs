use anyhow::Result;
use gio::prelude::*;
use gtk::{
	prelude::*,
	Application,
	ApplicationWindow,
	Window,
};

pub struct Scrkey {
	pub app: Application,
}

impl Scrkey {
	pub fn new(app_id: &str) -> Scrkey {
		let app = Application::builder().application_id(app_id).build();

		Scrkey { app }
	}

	pub fn run(&self) -> Result<()> {
		self.app.connect_activate(|app| Self::render(app));

		self.app.run();

		Ok(())
	}

	pub fn render(app: &Application) {
		let window = ApplicationWindow::builder()
			.application(app)
			.title("ScrKey.rs")
			.width_request(400)
			.height_request(60)
			.build();

		window.set_keep_above(true);
		window.set_decorated(false);
		window.set_resizable(false);
		window.stick();

		window.present();
	}
}
