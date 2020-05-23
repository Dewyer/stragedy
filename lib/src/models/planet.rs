use crate::models::resource::GameRes;
use crate::models::buildings::{resources,Building,resources::ResourceBuilding};
use crate::error::AuthError::BadEmail;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Planet
{
	#[serde(rename = "_id")]
	pub displayed_name:String,
	pub controlled_by:bson::oid::ObjectId,
	pub id: Option<bson::oid::ObjectId>,
	pub industrial_components:PlanetResource,
	pub computer_components:PlanetResource,
	pub organic_material:PlanetResource,
	pub manpower:Manpower,
	pub coordinate:PlanetCoordinate,

	//Buildings
	pub industrial_components_maker : resources::IndustrialCBuilding,
	pub computer_components_maker : resources::ComputerCBuilding,
	pub organic_material_maker : resources::OrganicMBuilding
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct PlanetInfoDto
{
	pub id:bson::oid::ObjectId,
	pub controlled_by:bson::oid::ObjectId,
	pub display_name:String,
	pub coordinates:PlanetCoordinate
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
	pub last_updated_timestamp:i64,
	pub income_per_hour:GameRes,
	pub storage_capacity: u64,
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
	pub fn new(controlled_by:bson::oid::ObjectId,coordinate:PlanetCoordinate) -> Self
	{
		Planet
		{
			controlled_by,
			displayed_name: "Home".to_string(),
			id: Some(bson::oid::ObjectId::new().unwrap()),
			industrial_components: PlanetResource::new(PlanetResourceType::IndustrialComponent),
			computer_components: PlanetResource::new(PlanetResourceType::ComputerComponent),
			organic_material: PlanetResource::new(PlanetResourceType::OrganicMaterial),
			manpower: Manpower::new(),
			coordinate,
			industrial_components_maker:resources::IndustrialCBuilding::new(),
			computer_components_maker: resources::ComputerCBuilding::new(),
			organic_material_maker : resources::OrganicMBuilding::new()
		}
	}

	pub fn get_cpy_id(&self) -> bson::oid::ObjectId
	{
		self.id.as_ref().unwrap().clone()
	}

	pub fn get_id(&self) ->&bson::oid::ObjectId
	{
		self.id.as_ref().unwrap()
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
			last_updated_timestamp:Utc::now().timestamp(),
			income_per_hour: match typ
			{
				PlanetResourceType::IndustrialComponent =>resources::IndustrialCBuilding::get_base_production(),
				PlanetResourceType::ComputerComponent =>resources::ComputerCBuilding::get_base_production(),
				PlanetResourceType::OrganicMaterial=> resources::OrganicMBuilding::get_base_production()
			},
			storage_capacity:1000
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

impl PlanetInfoDto
{
	pub fn from_planet(pl:&Planet) -> Self
	{
		Self
		{
			id: pl.id.as_ref().unwrap().clone(),
			display_name:pl.displayed_name.clone(),
			controlled_by:pl.controlled_by.clone(),
			coordinates: pl.coordinate.clone()
		}
	}
}
