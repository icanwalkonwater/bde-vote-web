use yew::html::Scope;
use yew::prelude::*;
use yew::virtual_dom::VNode;

use crate::vote_btn::VoteBtn;
use yew::services::ConsoleService;

pub struct ListPanel {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub class: String,
    pub open: Option<bool>,
    pub onclick: Callback<MouseEvent>,
}

pub enum Msg {
    SubmitVote(String, String),
}

impl Component for ListPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: Scope<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitVote(login, vote) => {
                ConsoleService::new().log(&format!("{} - {}", login, vote));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        // Only care about the open prop
        if props.open != self.props.open {
            self.props = props;
            true
        } else {
            self.props = props;
            false
        }
    }

    fn view(&self) -> VNode {
        html! {
            <div class={ self.classes() } onclick=&self.props.onclick>
                <div class="inner-container">
                    <span class="inner-title">{ &self.props.title }</span>
                    <VoteBtn visible={ self.props.open.unwrap_or(false) } onsubmit=self.link.callback(|l| Msg::SubmitVote(l, String::new()))/>
                </div>
            </div>
        }
    }
}

impl ListPanel {
    fn classes(&self) -> Classes {
        let mut classes = Classes::new();
        classes.push("side");
        classes.push(&self.props.class);

        if let Some(open) = self.props.open {
            if open {
                classes.push("open");
            } else {
                classes.push("reduced");
            }
        }

        classes
    }
}
