use yew::prelude::*;
use yew_router::prelude::*;

use routes::{routes_render, AppRoute};

mod pages;
mod routes;
mod utils;

#[allow(dead_code)]
struct App {
    link: ComponentLink<Self>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()> render = Router::render(routes_render)  />
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
