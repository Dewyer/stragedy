use super::{Building};
use crate::models::resource::GameRes;

pub trait ResourceBuilding
{
	fn get_base_production() -> GameRes;
}


#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct IndustrialCBuilding
{
	level:i32
}

impl Building for IndustrialCBuilding
{
	fn new() -> Self
	{
		Self
		{
			level:0
		}
	}

	fn get_level(&self) -> i32 {
		self.level
	}
}

impl ResourceBuilding for IndustrialCBuilding
{
	fn get_base_production() -> GameRes {
		GameRes::new(100)
	}
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct ComputerCBuilding
{
	level:i32
}

impl Building for ComputerCBuilding
{
	fn new() -> Self
	{
		Self
		{
			level:0
		}
	}

	fn get_level(&self) -> i32 {
		self.level
	}
}

impl ResourceBuilding for ComputerCBuilding
{
	fn get_base_production() -> GameRes {
		GameRes::new(70)
	}
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct OrganicMBuilding
{
	level:i32
}

impl Building for OrganicMBuilding
{
	fn new() -> Self
	{
		Self
		{
			level:0
		}
	}

	fn get_level(&self) -> i32 {
		self.level
	}
}

impl ResourceBuilding for OrganicMBuilding
{
	fn get_base_production() -> GameRes {
		GameRes::new(70)
	}
}

