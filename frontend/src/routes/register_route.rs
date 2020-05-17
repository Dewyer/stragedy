use yew::prelude::*;

pub struct RegisterRoute {
	link: ComponentLink<Self>
}

pub enum Msg {
	OnRegisterSubmit,
}

impl Component for RegisterRoute {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::OnRegisterSubmit => {}
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
            <div>
				<p>{"Hello travveler"}</p>
                <button onclick=self.link.callback(|_| Msg::OnRegisterSubmit)>{ "Register" }</button>
            </div>
        }
	}
}