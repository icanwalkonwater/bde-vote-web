use yew::prelude::*;
use yew::{ComponentLink, Component};
use yew::html::Scope;
use yew::virtual_dom::VNode;

pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: Scope<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <h1 class="test-class">{{ "Hello world !" }}</h1>
        }
    }
}
