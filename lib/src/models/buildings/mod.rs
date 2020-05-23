pub mod resources;
use crate::models::resource::GameRes;

pub trait Building
{
	fn new() -> Self;
	fn get_level(&self) -> i32;
}
