use yew::prelude::*;
use yew::Properties;
use yew::services::fetch::{FetchTask};
use crate::components::custom_button::CustomButton;
use crate::components::custom_text_input::CustomTextInput;
use crate::api;
use yew::services::ConsoleService;
use lib::requests::LoginPlayerRequest;
use lib::responses::LoginPlayerResponse;
use yew::services::StorageService;
use yew::services::storage::Area;
use yew_router::service::RouteService;
use yew_router::agent::{RouteAgent, RouteRequest};
use yew_router::prelude::*;

use crate::storage;
use crate::app::AppRoute;

pub struct LoginForm {
	link: ComponentLink<Self>,
	username:String,
	password:String,
	form_error:Option<String>,

	task:Option<FetchTask>,
	router:Box<dyn Bridge<RouteAgent>>
}

pub enum Msg {
	OnGotUsername(String),
	OnGotPassword(String),
	OnSubmit,
	StartFetch(FetchTask),
	OnGotError(String),
	OnSuccess(LoginPlayerResponse),
	NoOp
}

#[derive(Properties, Clone, PartialEq)]
pub struct LoginFormProps
{
}

impl Component for LoginForm {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		let cb = link.callback(|_|Msg::NoOp);
		let router = RouteAgent::bridge(cb);

		Self {
			link,
			username:"".to_string(),
			password:"".to_string(),
			form_error: None,
			task:None,
			router
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::OnGotUsername(val)=>self.username = val,
			Msg::OnGotPassword(val)=>self.password = val,
			Msg::OnSubmit=>self.submit_form(),
			Msg::StartFetch(task)=> self.task = Some(task),
			Msg::OnGotError(err)=>self.form_error = Some(err),
			Msg::OnSuccess(jwt)=>
			{
				ConsoleService::new().log(&format!("Jwt token : {}",&jwt.jwt));
				storage::save_auth_credentials(jwt);
				self.router.send(RouteRequest::ChangeRoute(Route::from(AppRoute::Game)));

			},
			Msg::NoOp=>{return false}
			_ => {}
		}
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
            <div class={"sig-border auth-form"}>
				<span class={"s-header"}>{"Login"}</span>
				<span class={"form-row"}>{"Username :"} <CustomTextInput on_change_text=self.link.callback(|val| Msg::OnGotUsername(val)) text={self.username.clone()}/></span>
				<span class={"form-row"}>{"Password :"} <CustomTextInput is_password={true} on_change_text=self.link.callback(|val| Msg::OnGotPassword(val)) text={self.password.clone()}/></span>
            	{ match self.form_error.as_ref()
            	{
            		Some(val) => html!{<span class={"form-error"}>{val.clone()}</span>},
            		None => html!{}
            	}}
            	<CustomButton is_disabled={!self.can_submit()} on_click=self.link.callback(|arg| Msg::OnSubmit) class={"form-submit"} text={"Login".to_string()}/>
            </div>
        }
	}
}

impl LoginForm
{
	fn submit_form(&self)
	{
		ConsoleService::new().log("Logining");
		let resp_task = api::login(LoginPlayerRequest{
			username: self.username.clone(),
			password: self.password.clone()
		},&self.link,|resp|{
			match resp
			{
				Ok(val)=> match val
				{
					Some(jwt) => Msg::OnSuccess(jwt),
					None => Msg::OnGotError("Server error".to_string())
				},
				Err(err)=> Msg::OnGotError(format!("Coudn't login : {}",err.to_string()))
			}
		});

		match resp_task
		{
			Ok(task) => self.link.send_message(Msg::StartFetch(task)),
			Err(e)=> self.link.send_message(Msg::OnGotError(format!("Couldn't send request: {}",e)))
		}
	}

	fn can_submit(&self) -> bool
	{
		self.username != "" && self.password != ""
	}
}
