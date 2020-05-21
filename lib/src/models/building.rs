use crate::models::resource::GameRes;

pub enum Building
{
	IndustrialFactory(i32),
	ComputerComponentFacility(i32),
	OrganicMatterHarvester(i32),
	SolarCore(i32)
}

impl Building
{
	pub fn get_base_production(&self) -> GameRes
	{
		GameRes::new(match self
		{
			Building::IndustrialFactory(v)=> 180,
			Building::ComputerComponentFacility(v)=> 90,
			Building::OrganicMatterHarvester(v)=> 90,
			Building::SolarCore(v)=> 100
		})
	}
}
