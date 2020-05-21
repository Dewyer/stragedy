#[macro_use] extern crate serde_derive;
pub mod requests;
pub mod responses;
pub mod regex;
pub mod error;
pub mod models;

#[derive(Serialize,Deserialize,Debug)]
pub struct ApiResponse<T,E>{
    pub content:Option<T>,
    pub error:E
}

/*
#[derive(Serialize,Deserialize,Debug)]
pub struct ApiEmptyResponse<E>
{
    pub error:E
}
*/
