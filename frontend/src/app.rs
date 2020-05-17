use yew::prelude::*;
use yew::services::{ConsoleService};
use yew::{ComponentLink};
use yew_router::{Switch,prelude::*};
use crate::routes::{register_route::RegisterRoute, index_route::IndexRoute};

pub struct App
{
    console:ConsoleService
}

#[derive(Switch, Debug,Clone)]
pub enum AppRoute {
    #[to = "/register"]
    Register,
    #[to = "/"]
    Index,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let cs= ConsoleService::new();
        App {
            console:cs
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Register => html!{<RegisterRoute />},
                                AppRoute::Index => html!{<IndexRoute />}
                            }
                        })
                />
            </div>
        }
    }
}
