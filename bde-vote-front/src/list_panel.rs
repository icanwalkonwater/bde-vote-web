use anyhow::Error;
use serde::Serialize;
use yew::format::Json;
use yew::html::Scope;
use yew::prelude::*;
use yew::services::{DialogService, FetchService};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::virtual_dom::VNode;
use dotenv_codegen::dotenv;

use crate::vote_btn::VoteBtn;

pub struct ListPanel {
    link: ComponentLink<Self>,
    props: Props,
    vote_task: Option<FetchTask>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub logo_url: String,
    pub vote: String,
    pub class: String,
    pub open: Option<bool>,
    pub onclick: Callback<MouseEvent>,
}

pub enum Msg {
    SubmitVote(String),
    SubmitOk,
    SubmitFailed,
}

impl Component for ListPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: Scope<Self>) -> Self {
        Self { link, props, vote_task: None }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::SubmitVote(login) => {
                let mut fetch = FetchService::new();

                #[derive(Serialize)]
                struct Payload {
                    pub who: String,
                    pub login: String,
                }

                let payload = Payload {
                    who: self.props.vote.clone(),
                    login,
                };

                let request = Request::post(dotenv!("VOTE_URL"))
                    .header("Content-Type", "application/json")
                    .body(Json(&payload))
                    .unwrap();

                self.vote_task = Some(fetch.fetch(
                    request,
                    self.link.callback(|res: Response<Result<String, Error>>| {
                        if res.status().is_success() {
                            Msg::SubmitOk
                        } else {
                            Msg::SubmitFailed
                        }
                    }),
                ).unwrap());

                true
            }
            Msg::SubmitOk => {
                DialogService::new().alert("Un mail de confirmation vous a été envoyer sur votre mail universitaire ! Il expirera dans 12h");
                false
            }
            Msg::SubmitFailed => {
                DialogService::new().alert("Une erreur est survenue ! Contactez un membre du BDE et merci de donnez l'heure à laquelle elle est survenue.");
                false
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
                    <img class="logo" src={ self.props.logo_url.clone() } />
                    <VoteBtn visible={ self.props.open.unwrap_or(false) } onsubmit=self.link.callback(|l| Msg::SubmitVote(l))/>
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
