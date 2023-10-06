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
use libc::{
	O_RDONLY,
	O_RDWR,
	O_WRONLY,
};
use std::{
	env::var,
	error::Error,
	fs::{
		File,
		OpenOptions,
	},
	io::stdout,
	os::unix::{
		fs::OpenOptionsExt,
		io::OwnedFd,
	},
	path::Path,
	process::Command,
};

pub struct Interface;

impl LibinputInterface for Interface {
	fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
		OpenOptions::new()
			.custom_flags(flags)
			.read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
			.write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
			.open(path)
			.map(|file| file.into())
			.map_err(|err| err.raw_os_error().unwrap())
	}
	fn close_restricted(&mut self, fd: OwnedFd) {
		drop(File::from(fd));
	}
}

pub fn new() -> Libinput {
	let mut input = Libinput::new_with_udev(Interface);
	input.udev_assign_seat("seat0").unwrap();

	return input;
}

pub fn detect_keypress(mut input: Libinput) {
    input.dispatch().unwrap();

    let keys: Vec<&str> = vec![];

    for event in input.clone().into_iter() {
        if let Event::Keyboard(Key(event)) = event {

        }
    }
}