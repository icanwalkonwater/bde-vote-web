/// Attempt to use yew_router but it's not working

use yew::{Component, ComponentLink, Html, html};
use yew_router::router::Router;
use yew_router::Switch;

use crate::home::Home;
use crate::confirmed_vote::ConfirmedVote;
use yew_router::service::RouteService;

pub struct App {
    link: ComponentLink<Self>,
    route_service: RouteService<AppRoute>,
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Home,
    #[to = "/confirmed"]
    ConfirmedVote,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_service = RouteService::<AppRoute>::new();

        App {
            link,
            route_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Home => html! { <Home/> },
                        AppRoute::ConfirmedVote => html! { <ConfirmedVote/> },
                    }
                })
            />
        }
    }
}
