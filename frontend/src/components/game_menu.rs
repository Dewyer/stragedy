use yew::prelude::*;
use yew::Properties;
use crate::components::game_menu_button::GameMenuButton;
use crate::app::AppRoute;

pub struct GameMenu {
	link: ComponentLink<Self>
}

pub enum Msg {

}

#[derive(Properties, Clone, PartialEq)]
pub struct GameMenuProps
{

}

impl Component for GameMenu {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
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

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
            <div class={"game-menu-container sig-border"}>
				<GameMenuButton title={"Planet".to_string()} to_route={AppRoute::Game}/>
				<GameMenuButton title={"Galaxy".to_string()} to_route={AppRoute::Game}/>
				<GameMenuButton title={"Military".to_string()} to_route={AppRoute::Game}/>
				<GameMenuButton title={"Research".to_string()} to_route={AppRoute::Register}/>
            </div>
        }
	}
}
