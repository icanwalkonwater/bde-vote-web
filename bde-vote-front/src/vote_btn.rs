use regex::Regex;
use web_sys::{Event, HtmlInputElement};
use yew::{Callback, Classes, Component, ComponentLink, html, NodeRef};
use yew::html::Scope;
use yew::macros::Properties;
use yew::services::DialogService;
use yew::virtual_dom::VNode;

pub struct VoteBtn {
    link: ComponentLink<Self>,
    props: Props,
    alert: DialogService,
    input_node: NodeRef,
    loading: bool,
    validation_regex: Regex,
}

pub enum Msg {
    SubmitVote(Event),
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or(false)]
    pub visible: bool,
    pub onsubmit: Callback<String>,
}

impl Component for VoteBtn {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: Scope<Self>) -> Self {
        Self {
            link,
            props,
            alert: DialogService::new(),
            input_node: NodeRef::default(),
            loading: false,
            validation_regex: Regex::new("^[a-z]{2,8}$").unwrap(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitVote(event) => {
                event.prevent_default();
                let el = self.input_node.cast::<HtmlInputElement>().unwrap();
                let value: String = el.value();

                if self.validation_regex.is_match(&value) {
                    self.loading = true;
                    self.props.onsubmit.emit(value);
                    true
                } else {
                    self.alert.alert("Ce n'est pas un login valide !");
                    el.set_value("");
                    false
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if props.visible != self.props.visible {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> VNode {
        let mut classes = Classes::new();
        classes.push("inline-flex");
        if !self.props.visible {
            classes.push("invisible-no-interaction");
        }

        html! {
            <form class=classes onsubmit=self.link.callback(|e| { Msg::SubmitVote(e) })>
                <input
                    ref=self.input_node.clone()
                    class="vote-field"
                    type="text"
                    disabled=self.loading
                    required=true
                    maxlength=8
                    placeholder="Login IUT"
                />
                <button
                    class="btn vote-btn"
                    type="submit"
                    disabled=self.loading
                >
                    { self.create_inner_btn() }
                </button>
            </form>
        }
    }
}

impl VoteBtn {
    fn create_inner_btn(&self) -> VNode {
        if !self.loading {
            html! {
                { "Voter" }
            }
        } else {
            html! {
                <svg class="loading-icon" viewBox="25 25 50 50">
                    <circle cx="50" cy="50" r="20"/>
                </svg>
            }
        }
    }
}
