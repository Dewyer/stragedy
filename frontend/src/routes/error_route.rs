use yew::prelude::*;
use crate::handlers::HandlerError;
use crate::handlers;

pub struct ErrorRoute {
	link: ComponentLink<Self>,
	error:HandlerError
}

pub enum Msg {

}

impl Component for ErrorRoute {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			error:HandlerError::load_last()
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
            <div class={"error-site-container"}>
				<h>{"Error: "}</h>
				<p>{self.error.err_str.clone()}</p>
				<p>{format!("At : {}",self.error.from_route)}</p>
				<p>{"If this problem persists contact the game operators."}</p>
            </div>
        }
	}
}
