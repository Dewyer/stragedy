---
to: src/components/<%= h.changeCase.snake(name) %>.rs
---
use yew::prelude::*;
use yew::Properties;

pub struct <%= name %> {
	link: ComponentLink<Self>
}

pub enum Msg {

}

#[derive(Properties, Clone, PartialEq)]
pub struct <%= name %>Props
{

}

impl Component for <%= name %> {
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
