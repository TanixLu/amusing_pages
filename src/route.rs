use yew::prelude::*;
use yew_router::prelude::*;

use crate::random_walk::RandomWalk;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/random_walk")]
    RandomWalk,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn Home() -> Html {
    html! {
        <ul class="item-list">
            <li><a href="random_walk">{"Random Walk test"}</a></li>
        </ul>
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::RandomWalk => html! {<RandomWalk />},
        Route::NotFound => html! {<h1>{"404 Not Found"}</h1>},
    }
}
