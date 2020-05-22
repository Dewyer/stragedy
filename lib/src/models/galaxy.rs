use crate::config::GalaxyConfig;
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use crate::models::planet::Planet;

pub struct PlanetDescriptor
{
	pub size_capacity:i64,
	pub occupied:bool
}

pub struct SolarSystem
{
	planets:Vec<PlanetDescriptor>
}

pub struct GalaxyUtil
{
	cfg:GalaxyConfig,
}

impl GalaxyUtil
{
	pub fn new(cfg:&GalaxyConfig) -> Self
	{
		Self
		{
			cfg:cfg.clone()
		}
	}

	pub fn get_planet_count_in_system(&self,solar_system:i64) -> i64
	{
		let mut rng:StdRng = StdRng::seed_from_u64((self.cfg.galaxy_planet_seed+solar_system) as u64);
		let size:i64 = rng.gen_range(10,20);
		size
	}

	pub fn get_solar_system_from_planets(&self,planets_in_system:Vec<Planet>) -> SolarSystem
	{
		unimplemented!();
		SolarSystem
		{
			planets:vec![]
		}
	}
}
