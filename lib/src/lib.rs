#[macro_use] extern crate serde_derive;

#[derive(Serialize,Deserialize)]
pub struct ApiResponse<T>{
    pub content:T,
    pub error:bool,
    pub status:String
}

#[derive(Serialize,Deserialize)]
pub struct ApiEmptyResponse
{
    pub error:bool,
    pub status:String
}