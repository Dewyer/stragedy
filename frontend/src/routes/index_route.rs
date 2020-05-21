use yew::prelude::*;
use crate::components::register_form::RegisterForm;
use crate::components::login_form::LoginForm;

pub struct IndexRoute {
	link: ComponentLink<Self>,
}

pub enum Msg {

}


impl Component for IndexRoute {
	type Message = Msg;
	type Properties = ();

	fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			_ => {}
		}
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender
	{
		false
	}

	fn view(&self) -> Html {
		html! {
            <div class={"index-container"}>
				<div class={"header sig-border"}><img src={"/static/icon.png"}/><span>{"STRAGEDY"}</span></div>
				<div class={"world-info-wrapper"}>
					<div class={"world-info sig-border"}>
						<span class={"s-header"}>{"World Info"}</span>
						{"Fuck there is no info yet :/"}
					</div>
				</div>
				<div class={"auth-container"}>
					<RegisterForm />
					<span class={"or"}>{"OR"}</span>
					<LoginForm />
				</div>
			</div>
        }
	}
}
