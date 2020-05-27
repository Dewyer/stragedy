use lib::requests::*;
use lib::{ApiResponse};
use lib::error::{AuthError,ApiError};
use http::{Request, StatusCode};
use http::Response;
use yew::services::fetch::{FetchService,FetchTask};
use yew::prelude::*;
use crate::api;
use lib::responses::*;
use http::method::Method;
use yew::services::StorageService;
use yew::services::storage::Area;
use stdweb::unstable::TryInto;
use crate::handlers;
use yew::format::Nothing;

pub const SERVER_URL:&str = "http://localhost:8000";
//https://envpvvdbty7wo.x.pipedream.net/
//pub const SERVER_URL:&str = "https://envpvvdbty7wo.x.pipedream.net";

fn start_building_request(route:&str,method:Method,auth:Option<String>) -> http::request::Builder
{
	let mut req = Request::builder().uri(format!("{}{}",SERVER_URL,route));
	req = req.method(method.clone());

	if let Some(auth_str) = auth
	{
		req = req.header("Authorization",&auth_str);
	}
	req
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

pub fn make_api_request<DataIn,ResponseData,ResponseErr,ModellT,Func>(route:&str,data:DataIn,method:Method,authed:bool,to_msg:Func,link:&ComponentLink<ModellT>) -> Result<FetchTask,ApiError>
where Func: Fn(Result<Option<ResponseData>,ResponseErr>) -> ModellT::Message +'static , ModellT : yew::Component, ResponseErr: lib::error::LibError, DataIn: serde::Serialize, ResponseData:Clone, ResponseData: serde::de::DeserializeOwned
{
	let auth_opt = if authed
	{
		let local_res = StorageService::new(Area::Local);
		let mut local = match local_res
		{
			Ok(ll)=> ll,
			Err(e)=>
			{
				return Err(ApiError::Other("No local storage".to_string()));
			}
		};
		let ls_res = local.restore::<Result<String,anyhow::Error>>("jwt");

		Some(ls_res.unwrap())
	}else
	{
		None
	};

	let do_fn = move |response:Response<Result<String,anyhow::Error>>| {
		if response.status().is_success() {
			let parsed = api::parse_api_response::<ResponseData,ResponseErr>(response);
			to_msg(parsed)
		}
		else if response.status() == StatusCode::from_u16(401).unwrap()
		{
			//Unauthed
			handlers::handle_unauthed();
			to_msg(Err(ResponseErr::get_unauthorized_error()))
		}
		else {
			to_msg(Err(ResponseErr::get_server_error()))
		}
	};
	let mut req_builder = api::start_building_request(route,method.clone(),auth_opt);
	let cb = link.callback(do_fn);
	let task_res:Result<FetchTask,anyhow::Error> = match method
	{
		Method::GET => {
			let req = req_builder.body(Nothing).map_err(|_| ApiError::Other("Couldn't construct get request".to_string()))?;
			FetchService::new().fetch(req, cb)
		},
		_=>
		{
			let body_str = serde_json::to_string(&data).map_err(|_| ApiError::Other("Coudn't serialize request body.".to_string()))?;
			req_builder = req_builder.header("Content-Type","application/json");
			let req = req_builder.body(Ok(body_str)).map_err(|_| ApiError::Other("Couldn't construct post request.".to_string()))?;
			FetchService::new().fetch(req,cb)
		}
	};
	task_res.map_err(|e| ApiError::Other(format!("Couldn't create request : {}",e)))
}

macro_rules! endpoint
{
	($name:ident,$route:expr,$method:expr,$authed:expr,$Dat:ty,$Out:ty,$Err:ty) => (
	pub fn $name<T,F>(data:$Dat,link:&ComponentLink<T>,to_msg:F) -> Result<FetchTask,ApiError>
	where F: Fn(Result<Option<$Out>,$Err>) -> T::Message +'static , T : yew::Component
	{
		make_api_request($route,data,$method,$authed,to_msg,link)
	});
}

endpoint!(register,"/api/register",Method::POST,false,CreatePlayerRequest,(),AuthError);
endpoint!(login,"/api/login",Method::POST,false,LoginPlayerRequest,LoginPlayerResponse,AuthError);
endpoint!(base_info,"/api/base",Method::GET,true,(),BaseResponse,ApiError);
