use yew::html::Scope;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::virtual_dom::VNode;
use yew::ComponentLink;

pub struct VoteBtn {
    link: ComponentLink<Self>,
    console: ConsoleService,
    title: String,
}

pub enum Msg {
    ClickVote,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
}

impl Component for VoteBtn {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: Scope<Self>) -> Self {
        Self {
            link,
            console: ConsoleService::new(),
            title: props.title,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        if let Self::Message::ClickVote = msg {
            self.console.log("Clicked !");
        }
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                <span>{ &self.title }</span>
                <button onclick=self.link.callback(|_| Msg::ClickVote)>{ "Click" }</button>
            </div>
        }
    }
}
