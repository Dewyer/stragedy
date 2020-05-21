use yew::prelude::*;
use yew::Properties;

pub struct CustomTextInput {
	link: ComponentLink<Self>,
	props:CustomTextInputProps
}

pub enum Msg {
	OnInput(String)
}

#[derive(Properties, Clone, PartialEq)]
pub struct CustomTextInputProps
{
	pub text:String,

	#[prop_or("".to_string())]
	pub placeholder:String,
	pub on_change_text:Callback<String>,

	#[prop_or(false)]
	pub is_password:bool
}

impl Component for CustomTextInput {
	type Message = Msg;
	type Properties = CustomTextInputProps;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			props,
			link
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg
		{
			Msg::OnInput(val)=>{
				self.props.on_change_text.emit(val);
			}
		}
		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.props = props;
		true
	}

	fn view(&self) -> Html {
		let typ = if self.props.is_password {"password"} else {"text"};
		html! {
            <input type={typ} placeholder={self.props.placeholder.clone()} class={"custom-text-input"} oninput=self.on_input() value={self.props.text.clone()}/>
        }
	}
}

impl CustomTextInput
{
	pub fn on_input(&self) -> Callback<InputData>
	{
		self.link.callback(|event:InputData| Msg::OnInput(event.value))
	}
}
