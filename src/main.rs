extern crate gdk;
extern crate gio;
extern crate gtk;
mod libs;

use libs::gui::Scrkey;

fn main() {
	let scrkey = Scrkey::new("org.scrkey.ScrKey");
	Scrkey::run(&scrkey).unwrap();
}
