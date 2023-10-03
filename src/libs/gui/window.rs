use anyhow::Result;
use sdl2::{
	event::Event,
	pixels::Color,
};
use std::time::Duration;

pub struct Scrkey {}

impl Scrkey {
	pub fn run() -> Result<()> {
		let context = sdl2::init().unwrap();
		let video = context.video().unwrap();

		let window = video.window("ScrKey", 400, 60).borderless().position(50, 50).build().unwrap();
		let mut canvas = window.into_canvas().build().unwrap();

		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
		canvas.present();

		let mut ev_pump = context.event_pump().unwrap();

		'render_loop: loop {
			for event in ev_pump.poll_iter() {
				match event {
					Event::Quit { .. } => break 'render_loop,
					_ => {}
				}
			}
			canvas.present();
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
		Ok(())
	}
}
