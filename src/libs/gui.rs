use crate::libs::{
	config::{
		parse_config,
		Config,
	},
	get_input,
};
use anyhow::Result;
use gdk::prelude::*;
use gtk::{
	prelude::*,
	Application,
};
use input::{
	event::{
		keyboard::{
			KeyState,
			KeyboardEventTrait,
		},
		KeyboardEvent::Key,
	},
	Event,
	Libinput,
	LibinputInterface,
};
use std::{
	io::{
		self,
		Read,
	},
	process::Command,
};
use tokio::runtime;

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
		gtk::init().unwrap();

		let config = parse_config();
		let window = gtk::Window::new(gtk::WindowType::Popup);
		let position = Self::get_position(&config);

		window.set_keep_above(true);
		window.set_decorated(false);
		window.set_resizable(false);
		window.move_(position.0, position.1);
		window.set_width_request(config.size.width as i32);
		window.set_height_request(config.size.height as i32);
		window.set_application(Some(app));
		window.set_title(config.general.title.as_str());
		window.stick();

		let input_label = gtk::Label::new(Some("hello world"));

		let vbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
		vbox.pack_start(&input_label, false, false, 0);

		window.add(&vbox);

		let input = get_input::new();

		let input_label_clone = input_label.clone();

		let rt = runtime::Builder::new_multi_thread()
			.worker_threads(2) // Adjust the number of worker threads as needed
			.enable_all()
			.build()
			.unwrap();

		rt.block_on(async {
			let mut keys = Vec::new();
			loop {
				input.clone().dispatch().unwrap();
				for event in input.clone().into_iter() {
					if let Event::Keyboard(Key(event)) = event {
						if event.key_state() == KeyState::Pressed {
							keys.push(event.key());
						}
						if event.key_state() == KeyState::Released {
							println!("{:?}", keys);
							input_label_clone.set_text(format!("{:?}", keys).as_str());
							if !keys.is_empty() {
								keys.clear();
							}
						}
					}
				}
				tokio::time::sleep(std::time::Duration::from_millis(10)).await;
			}
		});

		window.show_all();
		gtk::main();
	}

	pub fn get_display_size() -> (i32, i32) {
		let xwininfo_output = Command::new("xwininfo")
			.arg("-root")
			.stdout(std::process::Stdio::piped())
			.spawn()
			.unwrap();

		let grep_output = Command::new("grep")
			.args(&["-oP", r#"\-geometry \K[^+]+"#])
			.stdin(xwininfo_output.stdout.unwrap())
			.output()
			.unwrap();

		let output = String::from_utf8_lossy(&grep_output.stdout);
		let raw_dimensions: Vec<&str> = output.trim().split("x").collect();

		if raw_dimensions.len() == 2 {
			if let (Ok(width), Ok(height)) =
				(raw_dimensions[0].parse::<i32>(), raw_dimensions[1].parse::<i32>())
			{
				let dimensions = (width, height);
				return dimensions;
			} else {
				return (1920, 1080);
			}
		} else {
			return (1920, 1080);
		}
	}

	pub fn get_position(config: &Config) -> (i32, i32) {
		let display_size = Self::get_display_size();

		match config.position.anchor.as_str() {
			"top left" | "left top" => return (config.position.x as i32, config.position.y as i32),
			"top right" | "right top" => {
				return (
					(display_size.0 - (config.position.x + config.size.width) as i32),
					config.position.y as i32,
				)
			}
			"bottom left" | "left bottom" => {
				return (
					config.position.x as i32,
					(display_size.1 - (config.position.y + config.size.height) as i32),
				)
			}
			"bottom right" | "right bottom" => {
				return (
					(display_size.0 - (config.position.x + config.size.width) as i32),
					(display_size.1 - (config.position.y + config.size.height) as i32),
				)
			}
			&_ => {
				eprintln!("Unknown anchor.");
				return (0, 0);
			}
		}
	}
}
