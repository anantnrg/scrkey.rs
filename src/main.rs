mod libs;

use anyhow::Result;
use gtk::{
	glib,
	prelude::*,
	Application,
};
use libs::gui::window::create_window;

const APP_ID: &str = "scrkey";

fn main() -> Result<()> {
	let app = Application::builder().application_id(APP_ID).build();

	app.connect_activate(create_window);

	app.run();

	Ok(())
}
