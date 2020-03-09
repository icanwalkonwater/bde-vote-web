use yew::html::Scope;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};
use crate::vote_btn::VoteBtn;

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
            <div class="separator-container">
                <div class="side left">
                    <VoteBtn title="test"/>
                </div>
                <div class="side right"></div>
            </div>
        }
    }
}
