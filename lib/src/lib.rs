#[macro_use] extern crate serde_derive;
pub mod requests;
pub mod responses;
pub mod regex;
pub mod error;
pub mod models;
pub mod config;

#[derive(Serialize,Deserialize,Debug)]
pub struct ApiResponse<T,E>
{
    pub content:Option<T>,
    pub error:E
}

impl<T,E> ApiResponse<T,E>
where E:error::LibError
{
	pub fn from_result(res:Result<T,E>) -> ApiResponse<T,E>
	{
		match res
		{
			Ok(val) => Self
			{
				content:Some(val),
				error:E::get_no_error()
			},
			Err(ee)=> Self
			{
				content:None,
				error:ee
			}
		}
	}
}

/*
#[derive(Serialize,Deserialize,Debug)]
pub struct ApiEmptyResponse<E>
{
    pub error:E
}
*/
