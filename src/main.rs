mod route;
mod random_walk;

use yew::prelude::*;
use yew_router::prelude::*;

use route::*;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
