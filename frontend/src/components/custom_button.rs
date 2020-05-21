use yew::prelude::*;
use yew::Properties;

pub struct CustomButton {
	link: ComponentLink<Self>,
	props:CustomButtonInputProps
}

pub enum Msg {
	OnClick
}

#[derive(Properties, Clone, PartialEq)]
pub struct CustomButtonInputProps
{
	pub text:String,
	pub on_click:Option<Callback<()>>,

	#[prop_or("".to_string())]
	pub class:String,
	#[prop_or(false)]
	pub is_disabled:bool
}

impl Component for CustomButton {
	type Message = Msg;
	type Properties = CustomButtonInputProps;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			props
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::OnClick=> if let Some(cb) = self.props.on_click.as_ref() {cb.emit(())}
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
            <button disabled={self.props.is_disabled} class={format!("custom-button {}",self.props.class)} onclick=self.link.callback(|event| Msg::OnClick)>{self.props.text.clone()}</button>
        }
	}
}
