use std::{error::Error, fmt};
use lib::ApiEmptyResponse;

#[derive(Debug)]
pub struct ApiError
{
	status:String
}

impl ApiError
{
	pub fn get_status(&self) -> &str
	{
		&self.status
	}
}

impl From<Box<dyn Error>> for ApiError
{
	fn from(error: Box<dyn Error>) -> Self
	{
		ApiError
		{
			status:error.description().to_string()
		}
	}
}

impl Error for ApiError {}

impl fmt::Display for ApiError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "API error.")
	}
}
