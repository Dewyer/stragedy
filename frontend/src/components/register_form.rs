use yew::prelude::*;
use crate::components::custom_text_input::CustomTextInput;
use crate::components::custom_button::CustomButton;
use yew::services::{ConsoleService};
use yew::services::{FetchService};
use yew::services::fetch::{FetchTask,FetchOptions};
use http::request::Request;
use http::Response;
use crate::api;
use lib::requests::CreatePlayerRequest;
use yew::format::{Json, Nothing};
use lib::error::AuthError;


pub struct RegisterForm {
	link: ComponentLink<Self>,
	username:String,
	email:String,
	password:String,
	confirm_password:String,
	form_error:Option<String>,
	success_text:Option<String>,

	fetch:FetchService,
	task:Option<FetchTask>,
	console:ConsoleService
}

pub enum Msg {
	OnGotUsername(String),
	OnGotEmail(String),
	OnGotPassword(String),
	OnGotConfirmPassword(String),
	OnGotRegisterResult(String),
	OnSubmit,
	StartFetch(FetchTask)
}

impl Component for RegisterForm {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			username:"".to_string(),
			email:"".to_string(),
			password:"".to_string(),
			confirm_password:"".to_string(),
			fetch:FetchService::new(),
			console: ConsoleService::new(),
			form_error:None,
			task:None,
			success_text:None
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::OnGotUsername(new_username)=>self.username = new_username,
			Msg::OnGotEmail(val) => self.email = val,
			Msg::OnGotPassword(val)=>self.password = val,
			Msg::OnGotConfirmPassword(val)=> self.confirm_password = val,
			Msg::OnSubmit=>{
				self.submit_form();
			}
			Msg::OnGotRegisterResult(res)=> {
				self.success_text = Some(res);
				self.task = None;
			},
			Msg::StartFetch(task) => self.task = Some(task),
			_ => {}
		}
		self.form_error = self.get_form_submitability_error();
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
            <div class={"sig-border auth-form"}>
				<span class={"s-header"}>{"Register"}</span>
				<span class={"form-row"}>{"Username :"} <CustomTextInput on_change_text=self.on_got_username() text={self.username.clone()}/></span>
				<span class={"form-row"}>{"Email :"} <CustomTextInput on_change_text=self.on_got_email() text={self.email.clone()}/></span>
				<span class={"form-row"}>{"Password :"} <CustomTextInput is_password={true} on_change_text=self.on_got_password() text={self.password.clone()}/></span>
				<span class={"form-row"}>{"Confirm Password :"} <CustomTextInput is_password={true} on_change_text=self.on_got_confirm_password() text={self.confirm_password.clone()}/></span>
            	{ match self.form_error.as_ref()
            	{
            		Some(val) => html!{<span class={"form-error"}>{val.clone()}</span>},
            		None => html!{}
            	}}
            	{ match self.success_text.as_ref() {
            		Some(val) => html!{<span class={"form-success"}>{val.clone()}</span>},
					None => html!{}
            	}}
            	<CustomButton is_disabled={self.form_error.is_some()} on_click=self.link.callback(|arg| Msg::OnSubmit) class={"form-submit"} text={"Register".to_string()}/>
            </div>
        }
	}
}

impl RegisterForm
{
	pub fn submit_form(&mut self)
	{
		self.console.log("mi a fasz log");
		let reg_task = api::register(CreatePlayerRequest{
			username: self.username.clone(),
			email: self.email.clone(),
			password: self.password.clone()
		},&self.link,|resp| {
			match resp.as_ref()
			{
				Ok(_)=> Msg::OnGotRegisterResult("Successfully registered, now you can log in!".to_string()),
				Err(e)=>Msg::OnGotRegisterResult(format!("Error while registering : {}",e.to_string()))
			}
		});

		if let Ok(task) = reg_task
		{
			self.link.send_message(Msg::StartFetch(task));
		}
	}

	pub fn get_form_submitability_error(&self) -> Option<String>
	{
		let pwd_regex = lib::regex::get_password_regex();
		let email_regex = lib::regex::get_email_regex();
		if self.username.len() < 3 || self.username.len() > 20
		{
			Some("Username must be at 3-20 characters long.".to_string())
		}
		else if self.password != self.confirm_password
		{
			Some("Passwords do not match!".to_string())
		}
		else if !pwd_regex.is_match(&self.password)
		{
			Some("Password must be more than 6 characters long and contain numbers and letters.".to_string())
		}
		else if !email_regex.is_match(&self.email)
		{
			Some("Invalid e-mail address.".to_string())
		}
		else if self.task.is_some()
		{
			Some("Loading ...".to_string())
		}
		else
		{
			None
		}
	}

	pub fn on_got_username(&self) -> Callback<String>
	{
		self.link.callback(|val| Msg::OnGotUsername(val))
	}

	pub fn on_got_email(&self) -> Callback<String>
	{
		self.link.callback(|val| Msg::OnGotEmail(val))
	}

	pub fn on_got_password(&self) -> Callback<String>
	{
		self.link.callback(|val| Msg::OnGotPassword(val))
	}

	pub fn on_got_confirm_password(&self) -> Callback<String>
	{
		self.link.callback(|val| Msg::OnGotConfirmPassword(val))
	}
}
