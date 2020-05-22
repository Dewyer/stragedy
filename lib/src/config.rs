
#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct GalaxyConfig {
	pub version: String,
	pub no_solar_systems:i64,
	pub galaxy_planet_seed:i64
}

impl ::std::default::Default for GalaxyConfig {
	fn default() -> Self {
		Self {
			version: "alpha".to_string(),
			no_solar_systems:100,
			galaxy_planet_seed:420
		}
	}
}
