use yew::{ html, Html, Component, Context };

pub struct Item {
    label: String,
    url: String
}

impl Item {
    pub fn new(label: String, url: String) -> Self {
        Item {
            label,
            url
        }
    }

    pub fn render(&self) -> Html {
        html! {
            <li>{self.label.clone()}{" - "}{self.url.clone()}</li>
        }
    }
}

impl Component for Item {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Item {
            label: String::from("label"),
            url: String::from("url")
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.render()
    }
}