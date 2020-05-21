use yew::prelude::*;

pub struct GameRoute {
	link: ComponentLink<Self>
}

pub enum Msg {

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

	fn view(&self) -> Html {
		html! {
            <div>
				<p>{"Game !"}</p>
            </div>
        }
	}
}
