use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, sierpinski::Sierpinski};

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/sierpinski"]
    Sierpinski,

    #[to = "/"]
    Home,
}

pub fn routes_render(switch: AppRoute) -> Html {
    match switch {
        AppRoute::Home => html! {<Home />},

        AppRoute::Sierpinski => html! { <Sierpinski /> },
    }
}
