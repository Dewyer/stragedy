use crate::models::resource::GameRes;

#[derive(Serialize, Deserialize, Debug)]
pub struct Planet
{
	#[serde(rename = "_id")]
	pub id: Option<bson::oid::ObjectId>,
	pub industrial_components:PlanetResource,
	pub computer_components:PlanetResource,
	pub organic_material:PlanetResource,
	pub manpower:Manpower

}


#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct PlanetResourceData
{
	pub base_value:GameRes,
	pub last_updated_tick:i64,
	pub income_per_hour:GameRes,
}

#[derive(Serialize,Deserialize,Debug)]
pub enum PlanetResource
{
	IndustrialComponent(PlanetResourceData),
	ComputerComponent(PlanetResourceData),
	OrganicMaterial(PlanetResourceData)
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Manpower
{
	pub current_value:GameRes
}


impl Planet
{

}
