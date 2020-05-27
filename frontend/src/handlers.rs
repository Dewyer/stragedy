use yew_router::service::RouteService;
use yew_router::agent::{RouteAgent, RouteRequest};
use yew::{Bridged, Callback};
use yew_router::prelude::*;
use crate::app::AppRoute;
use yew::services::{ConsoleService, StorageService};
use lib::error::LibError;
use std::error::Error;
use wasm_bindgen::__rt::core::fmt::Formatter;
use yew::services::storage::Area;
use serde::{Serialize,Deserialize};

#[derive(Clone,Serialize,Deserialize)]
pub struct HandlerError
{
	pub err_str:String,
	pub from_route:String,
}

impl HandlerError
{
	fn get_storage() -> Option<StorageService>
	{
		StorageService::new(Area::Local).ok()
	}

	pub fn to_json(&self) -> String
	{
		serde_json::to_string(self).unwrap()
	}

	pub fn from_json(src:&str) -> Option<Self>
	{
		serde_json::from_str(src).ok()
	}

	pub fn save(&self) -> Option<()>
	{
		let mut storage = Self::get_storage()?;
		storage.store("last_error",Ok(self.to_json()));
		Some(())
	}

	fn try_load() -> Option<Self>
	{
		let mut storage = Self::get_storage()?;
		let exp_str = storage.restore::<Result<String,anyhow::Error>>("last_error").ok()?;
		HandlerError::from_json(&exp_str)
	}

	pub fn load_last() -> Self
	{
		Self::try_load().map_or(Self::default(),|val| val)
	}
}

impl Default for HandlerError
{
	fn default() -> Self {
		Self
		{
			err_str:"unknown or no previus errors.".to_string(),
			from_route:"unknown place".to_string()
		}
	}
}

pub trait HandleUnwrap<T,E>
where E: Error + LibError
{
	fn unwrap_with_handle(self) -> T;
}

impl<T,E> HandleUnwrap<T,E> for Result<T,E>
where E: Error + LibError
{
	fn unwrap_with_handle(self) -> T {
		match self
		{
			Ok(val)=>val,
			Err(ee)=>
				{
					let handle_err = HandlerError
					{
						err_str:format!("{}",ee),
						from_route:web_sys::window().unwrap().location().pathname().unwrap(),
					};
					handle_err.save();
					web_sys::window().unwrap().location().set_href("/error");
					panic!();
				}
		}
	}
}

pub fn handle_unauthed()
{
	ConsoleService::new().error("User is not authroized, sending to index.");
	let mut router:Box<dyn yew::Bridge<yew_router::agent::RouteAgent<()>>> = RouteAgent::bridge(Callback::noop());
	router.send(RouteRequest::ChangeRoute(Route::from(AppRoute::Index)));
}
