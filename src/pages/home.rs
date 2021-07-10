use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

pub struct Home;

#[allow(dead_code)]
#[allow(unused_variables)]
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <RouterAnchor<AppRoute> route=AppRoute::Sierpinski>
                    { "Sierpinski" }
                </RouterAnchor<AppRoute>>
            </div>
        }
    }
}
