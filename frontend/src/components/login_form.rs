use yew::prelude::*;
use yew::Properties;

pub struct LoginForm {
	link: ComponentLink<Self>
}

pub enum Msg {

}

#[derive(Properties, Clone, PartialEq)]
pub struct LoginFormProps
{

}

impl Component for LoginForm {
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
            <div>
				<p>{"Hello !"}</p>
            </div>
        }
	}
}
