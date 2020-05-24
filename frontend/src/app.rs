use yew::prelude::*;
use yew::services::{ConsoleService};
use yew::{ComponentLink};
use yew_router::{Switch,prelude::*};
use crate::routes::{register_route::RegisterRoute, index_route::IndexRoute,game_route::GameRoute};
use yew_router::switch::Permissive;

pub struct App
{
    console:ConsoleService
}

#[derive(Switch, Debug,Clone,PartialEq)]
pub enum AppRoute {
	#[to="/game"]
	Game,
    #[to = "/register"]
    Register,
	#[to= "/404"]
	PageNotFound(Permissive<String>),
    #[to = "/"]
    Index
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
								AppRoute::Game => html!{<GameRoute />},
                                AppRoute::Register => html!{<RegisterRoute />},
								AppRoute::PageNotFound(Permissive(Some(rt))) => html!{<p class={"four-o-four"}>{format!("Page not found : {} :/",rt.clone())}</p>},
								AppRoute::PageNotFound(Permissive(None)) => html!{<p class={"four-o-four"}>{"Page not found!"}</p>},
                                AppRoute::Index => html!{<IndexRoute />}
                            }
                        })
                        redirect = Router::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route)))
                        })
                />
            </div>
        }
    }
}
