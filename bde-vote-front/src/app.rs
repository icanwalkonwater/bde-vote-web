use crate::list_panel::ListPanel;
use yew::html::Scope;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink};

pub struct App {
    link: ComponentLink<Self>,
    side_open: SideOpen,
}

#[derive(PartialEq, Eq)]
pub enum SideOpen {
    Left,
    Right,
    None,
}

pub enum Msg {
    ClickLeft,
    ClickRight,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: Scope<Self>) -> Self {
        Self {
            link,
            side_open: SideOpen::None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ClickLeft if self.side_open != SideOpen::Left => {
                self.side_open = SideOpen::Left;
                true
            }
            Msg::ClickRight if self.side_open != SideOpen::Right => {
                self.side_open = SideOpen::Right;
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> VNode {
        let left_open = if self.side_open == SideOpen::Left {
            Some(true)
        } else {
            None
        };
        let right_open = if self.side_open == SideOpen::Right {
            Some(true)
        } else {
            None
        };

        html! {
            <div class="separator-container">
                <ListPanel class="left" open={ left_open } title="αOS" onclick={ self.link.callback(|_| Msg::ClickLeft) }/>
                <ListPanel class="right" open={ right_open } title="4ème Compilation" onclick={ self.link.callback(|_| Msg::ClickRight) }/>
            </div>
        }
    }
}
