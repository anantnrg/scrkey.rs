use anyhow::Result;
use gio::prelude::*;
use gtk::{
	prelude::*,
	Application,
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

	pub fn run(app: Application) -> Result<()> {
		app.run();

		Ok(())
	}
}
