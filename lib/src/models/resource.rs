use serde::{Serialize, Serializer};

//Game resource
#[derive(Serialize, Deserialize, Debug,Clone,Copy)]
pub struct GameRes
{
	inner:i64
}

impl GameRes
{
	pub fn new(val:i64) -> Self
	{
		GameRes
		{
			inner:val
		}
	}
}
