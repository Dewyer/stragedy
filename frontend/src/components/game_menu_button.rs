use yew::prelude::*;
use yew::Properties;
use crate::components::custom_button::CustomButton;
use yew_router::agent::{RouteAgent, RouteRequest};
use yew_router::prelude::*;
use crate::app::AppRoute;

pub struct GameMenuButton {
	link: ComponentLink<Self>,
	props:GameMenuButtonProps,
}

pub enum Msg {
}

#[derive(Properties, Clone, PartialEq)]
pub struct GameMenuButtonProps
{
	pub title:String,
	pub to_route:AppRoute
}

impl Component for GameMenuButton {
	type Message = Msg;
	type Properties = GameMenuButtonProps;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			props,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			_ => {}
		}
		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender
	{
		self.props = props;
		true
	}

	fn view(&self) -> Html {
		html! {
			<RouterButton<AppRoute> route={self.props.to_route.clone()} classes={"custom-button menu-button"}> {&self.props.title}</RouterButton<AppRoute>>
        }
	}
}
