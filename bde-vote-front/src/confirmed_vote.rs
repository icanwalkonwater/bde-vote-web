use yew::{ComponentLink, Component, Html, html};

pub struct ConfirmedVote;

impl Component for ConfirmedVote {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ConfirmedVote
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style="margin: auto">
                <h1>{ "Votre vote a bien été confirmer !" }</h1>
            </div>
        }
    }
}