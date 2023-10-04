use serde_derive::Deserialize;
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Deserialize)]
pub struct GeneralConfig {
	#[default(false)]
	pub mod_keystrokes_only: bool,
	#[default("minimal")]
	pub style: String,
	#[default(true)]
	pub highlight_mods: bool,
}

#[derive(Debug, SmartDefault, Deserialize)]
pub struct FontConfig {
	#[default("JetBrains Mono Nerd Font")]
	pub font: String,
	#[default(16)]
	pub font_size: u32,
}

#[derive(Debug, SmartDefault, Deserialize)]
pub struct PosConfig {
	#[default(50)]
	pub x: u32,
	#[default(50)]
	pub y: u32,
	#[default("bottom left")]
	pub anchor: String,
}

#[derive(Debug, SmartDefault, Deserialize)]
pub struct SizeConfig {
	#[default(400)]
	pub width: u32,
	#[default(60)]
	pub height: u32,
}

#[derive(Debug, SmartDefault, Deserialize)]
pub struct ColorsConfig {
	#[default("#181825")]
	pub background: String,
	#[default("#cdd6f4")]
	pub foreground: String,
	#[default("#f38ba8")]
	pub highlight: String,
}

#[derive(Debug, SmartDefault, Deserialize)]
pub struct Config {
	pub general: GeneralConfig,
	pub font: FontConfig,
	pub position: PosConfig,
	pub size: SizeConfig,
	pub colors: ColorsConfig,
}

pub fn parse_config() -> Config {
	let xdg_dirs = xdg::BaseDirectories::with_prefix("scrkey").unwrap();
	let config_path = xdg_dirs.place_config_file("config.toml").unwrap();

	let config_str = std::fs::read_to_string(config_path).unwrap();

	let config: Config = toml::from_str(&config_str).unwrap();

	return config;
}
