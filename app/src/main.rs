use yew::{html, Html, Component, Context};

struct Main {}

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Main {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{ "hello, world!" }</div>
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
