use lib::requests::CreatePlayerRequest;
use lib::{ApiResponse};
use lib::error::{AuthError,ApiError};
use http::Request;
use http::Response;
use yew::services::fetch::{FetchService,FetchTask};
use yew::prelude::*;
use crate::api;

pub const SERVER_URL:&str = "http://localhost:8000";
//https://envpvvdbty7wo.x.pipedream.net/
//pub const SERVER_URL:&str = "https://envpvvdbty7wo.x.pipedream.net";

pub fn make_request<T>(data:T,route:&str) -> Result<Request<Result<String,anyhow::Error>>,serde_json::error::Error>
where T:serde::Serialize
{
	let body_str = serde_json::to_string(&data)?;
	Ok(Request::builder().method(http::method::Method::POST).uri(format!("{}{}",SERVER_URL,route)).header("Content-Type","application/json").body(Ok(body_str)).unwrap())
}

fn parse_api_response<D,E>(resp:Response<Result<String,anyhow::Error>>) -> Result<Option<D>,E>
where E: lib::error::LibError, D: serde::de::DeserializeOwned, D: Clone
{
	if resp.status().is_success()
	{
		let bdy = resp.body();
		if let Err(k) = bdy
		{
			return Err(E::get_server_error());
		}
		let bdy_s = bdy.as_ref().unwrap();
		let bdy_data = serde_json::from_str::<ApiResponse<D,E>>(&bdy_s);
		if let Ok(body) = bdy_data.as_ref() {
			return if body.error == E::get_no_error()
			{
				Ok(body.content.clone())
			} else {
				Err(body.error.clone())
			}
		}
	}
	return Err(E::get_server_error())
}

pub fn make_api_request<DataIn,ResponseData,ResponseErr,ModellT,Func>(route:&str,data:DataIn,to_msg:Func,link:&ComponentLink<ModellT>) -> Result<FetchTask,ApiError>
where Func: Fn(Result<Option<ResponseData>,ResponseErr>) -> ModellT::Message +'static , ModellT : yew::Component, ResponseErr: lib::error::LibError, DataIn: serde::Serialize, ResponseData:Clone, ResponseData: serde::de::DeserializeOwned
{
	let req_res = api::make_request(data,route);
	if let Ok(req) = req_res
	{
		let task:Result<FetchTask,anyhow::Error> = FetchService::new().fetch(
			req,
			link.callback(move |response:Response<Result<String,anyhow::Error>>| {
				if response.status().is_success() {
					let parsed = api::parse_api_response::<ResponseData,ResponseErr>(response);
					to_msg(parsed)
				} else {
					to_msg(Err(ResponseErr::get_server_error()))
				}
			})
		);

		if let Ok(ta) = task {
			return Ok(ta);
		}
	}
	Err(ApiError::ServerError)
}

pub fn register<T,F>(data:CreatePlayerRequest,link:&ComponentLink<T>,to_msg:F) -> Result<FetchTask,ApiError>
where F: Fn(Result<Option<()>,AuthError>) -> T::Message +'static , T : yew::Component
{
	make_api_request("/api/register",data,to_msg,link)
}
