#[macro_use] extern crate serde_derive;
pub mod requests;
pub mod responses;

#[derive(Serialize,Deserialize)]
pub struct ApiResponse<T>{
    pub content:Option<T>,
    pub error:bool,
    pub status:String
}

#[derive(Serialize,Deserialize)]
pub struct ApiEmptyResponse
{
    pub error:bool,
    pub status:String
}
