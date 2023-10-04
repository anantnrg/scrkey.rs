use anyhow::Result;
use gdk::prelude::*;
use gtk::{
	prelude::*,
	Application,
};

use crate::libs::config::parse_config;

pub struct Scrkey {
	pub app: Application,
	// pub config: Config,
}

impl Scrkey {
	pub fn new(app_id: &str) -> Scrkey {
		let app = Application::builder().application_id(app_id).build();
		parse_config();

		Scrkey { app }
	}

	pub fn run(&self) -> Result<()> {
		self.app.connect_activate(|app| Self::render(app));

		self.app.run();

		Ok(())
	}

	pub fn render(app: &Application) {
		let window = gtk::Window::new(gtk::WindowType::Popup);

		window.set_keep_above(true);
		window.set_decorated(false);
		window.set_resizable(false);
		window.move_(50, 50);
		window.set_width_request(400);
		window.set_height_request(60);
		window.set_application(Some(app));
		window.set_title("ScrKey.rs");
		window.stick();

		window.present();
	}
}
