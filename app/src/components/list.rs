use yew::{ html, Component };
use crate::components::Item;

pub struct List {
    items: Vec<Item>
}

impl Component for List {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let item = Item::new(String::from("Yolo"), String::from("Test"));
        let item2 = Item::new(String::from("Yolo2"), String::from("Test2"));

        List {
            items: vec![item, item2]
        }
    }
 
    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <ul>{ for self.items.iter().map(|p| p.render()) }</ul>
        }
    }
}