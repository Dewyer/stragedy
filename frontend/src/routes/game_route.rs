use yew::prelude::*;
use crate::components::game_menu::GameMenu;
use crate::api;
use lib::responses::BaseResponse;
use yew::services::ConsoleService;
use crate::handlers::HandleUnwrap;
use yew::services::fetch::{FetchTask};

pub struct GameRoute {
	link: ComponentLink<Self>,
	task:Option<FetchTask>
}

pub enum Msg {
	OnGotBaseData(BaseResponse),
	StartFetch(FetchTask),
	NoOp
}

impl Component for GameRoute {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		Self {
			link,
			task:None
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::StartFetch(task) => self.task = Some(task),
			_ => {}
		}
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn rendered(&mut self, first_render: bool) {
		if first_render {
			ConsoleService::new().log("Loading base info.");
			let task_res = api::base_info((),&self.link,|resp|
			{
				ConsoleService::new().log("Base info got.");
				Msg::OnGotBaseData(resp.unwrap_with_handle().unwrap())
			});
			self.link.send_message(Msg::StartFetch(task_res.unwrap_with_handle()));
		}
	}

	fn view(&self) -> Html {
		html! {
            <div class={"game-container"}>
				<div>
					<img src={"/static/icon.png"} class={"icon-img"}/>
					<GameMenu />
				</div>
				<div class={"middle-panel"}>

				</div>
            </div>
        }
	}
}
