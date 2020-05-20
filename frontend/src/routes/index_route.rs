use yew::prelude::*;

pub struct IndexRoute {
	link: ComponentLink<Self>
}

pub enum Msg {

}

impl Component for IndexRoute {
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
		// Should only return "true" if new properties are different to
		// previously received properties.
		// This component has no properties so we will always return "false".
		false
	}

	fn view(&self) -> Html {
		html! {
            <div class={"index-container"}>
				<div class={"header sig-border"}><img src={"/static/icon.png"}/><span>{"STRAGEDY"}</span></div>
				<div class={"auth-container"}>

				</div>
			</div>
        }
	}
}
