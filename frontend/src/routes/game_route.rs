use yew::prelude::*;
use crate::components::game_menu::GameMenu;
use crate::api;
use lib::responses::BaseResponse;

pub struct GameRoute {
	link: ComponentLink<Self>
}

pub enum Msg {
	OnGotBaseData(BaseResponse)
}

impl Component for GameRoute {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self
	{

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

	fn rendered(&mut self, first_render: bool) {
		if first_render {
			api::base_info((),&self.link,|resp|
			{
				panic!();
			})
		}
	}

	fn view(&self) -> Html {
		html! {
            <div class={"game-container"}>
				<div>
					<img src={"/static/icon.png"} class={"icon-img"}/>
					<GameMenu />
				</div>
				<div class={"middle-panel"}></div>
            </div>
        }
	}
}
