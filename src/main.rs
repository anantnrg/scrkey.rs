extern crate sdl2;
mod libs;

use libs::gui::window::Scrkey;

pub fn main() {
	Scrkey::run().unwrap();
}
