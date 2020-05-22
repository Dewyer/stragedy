use crate::models::resource::GameRes;
use crate::models::building::Building;
use crate::error::AuthError::BadEmail;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Planet
{
	#[serde(rename = "_id")]
	pub id: Option<bson::oid::ObjectId>,
	pub industrial_components:PlanetResource,
	pub computer_components:PlanetResource,
	pub organic_material:PlanetResource,
	pub manpower:Manpower,
	pub coordinate:PlanetCoordinate
}

#[derive(Serialize, Deserialize, Debug,PartialEq,Clone)]
pub struct PlanetCoordinate
{
	pub solar_system:i64,
	pub planet_index:i64
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct PlanetResource
{
	pub type_v:PlanetResourceType,
	pub base_value:GameRes,
	pub last_updated_tick:i64,
	pub income_per_hour:GameRes,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum PlanetResourceType
{
	IndustrialComponent,
	ComputerComponent,
	OrganicMaterial
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Manpower
{
	pub current_value:GameRes
}

impl Planet
{
	pub fn new(coordinate:PlanetCoordinate) -> Self
	{
		Planet
		{
			id: Some(bson::oid::ObjectId::new().unwrap()),
			industrial_components: PlanetResource::new(PlanetResourceType::IndustrialComponent),
			computer_components: PlanetResource::new(PlanetResourceType::ComputerComponent),
			organic_material: PlanetResource::new(PlanetResourceType::OrganicMaterial),
			manpower: Manpower::new(),
			coordinate
		}
	}
}

impl PlanetCoordinate
{
	pub fn new(system:i64,planet:i64) -> Self
	{
		Self
		{
			solar_system:system,
			planet_index:planet
		}
	}
}

impl PlanetResource
{
	pub fn new(typ:PlanetResourceType) -> Self
	{
		Self
		{
			type_v:typ.clone(),
			base_value:GameRes::new(match typ
			{
				PlanetResourceType::IndustrialComponent => 300,
				PlanetResourceType::ComputerComponent =>150,
				PlanetResourceType::OrganicMaterial => 20
			}),
			last_updated_tick:Utc::now().timestamp(),
			income_per_hour: match typ
			{
				PlanetResourceType::IndustrialComponent => Building::IndustrialFactory(0).get_base_production(),
				PlanetResourceType::ComputerComponent =>Building::ComputerComponentFacility(0).get_base_production(),
				PlanetResourceType::OrganicMaterial=> Building::OrganicMatterHarvester(0).get_base_production()
			}
		}
	}
}

impl Manpower
{
	pub fn new()->Self
	{
		Manpower
		{
			current_value:GameRes::new(120i64)
		}
	}
}
