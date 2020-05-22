use lib::models::planet::PlanetCoordinate;
use lib::error::ApiError;
use crate::repos::Repo;
use crate::services;
use rand::Rng;
use lib::models::galaxy::GalaxyUtil;

pub fn get_starter_planet_coordinate(repo:&Repo) -> Result<PlanetCoordinate,ApiError>
{
	let cfg = services::config::get_config();
	let gutil = GalaxyUtil::new(cfg);

	for solar_diff in 0.. cfg.no_solar_systems
	{
		let rnd_side = rand::thread_rng().gen_range(0,2)-1;
		let pick_order = vec![cfg.no_solar_systems/2 + rnd_side*solar_diff,cfg.no_solar_systems/2 -rnd_side*solar_diff];
		let mut place_found:Option<PlanetCoordinate> = None;

		for at_ss in pick_order
		{
			let planet_in_system = repo.planet_repo.find_all_like(doc!{"coordinate.solar_system":at_ss})?;
			let no_planets = gutil.get_planet_count_in_system(at_ss);
			let mut eligible_indexes:Vec<i64> = Vec::new();
			for ii in 4..(no_planets-3)
			{
				if ii % 2 == 0
				{
					if !planet_in_system.iter().any(|pp| pp.coordinate.planet_index == ii)
					{
						eligible_indexes.push(ii);
					}
				}
			}
			if eligible_indexes.len() == 0
			{
				continue;
			}

			let pick = eligible_indexes[rand::thread_rng().gen_range(0,eligible_indexes.len())];
			place_found = Some(PlanetCoordinate::new(at_ss,pick));
		}
		if let Some(planet_coord) = place_found
		{
			return Ok(planet_coord);
		}

	}
	Err(ApiError::ServerError)
}
