use lib::config::GalaxyConfig;
use std::fs;
use std::path::Path;
use std::env;
use confy::load;

lazy_static!
{
	static ref CONFIG:GalaxyConfig = load_config();
}

fn load_config() -> GalaxyConfig
{
	let mut ph = env::current_exe().expect("No parent dir found ??");
	ph.set_file_name("config");
	ph.set_extension("json");
	println!("Config file path: {}",ph.to_str().unwrap());
	if ph.exists()
	{
		let contents = fs::read_to_string(ph.clone()).expect("Couldn't read from config file!");
		let cf = serde_json::from_str::<GalaxyConfig>(&contents).expect("Couldn't deserialize galaxy config.");
		cf

	}
	else
	{
		let def_conf = GalaxyConfig::default();
		fs::write(ph.clone(),serde_json::to_string(&def_conf).unwrap()).expect("Couldn't save config file.");
		def_conf
	}
}

pub fn get_config() -> &'static GalaxyConfig
{
	&*CONFIG
}
