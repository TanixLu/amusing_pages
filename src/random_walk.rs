use std::f64;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component]
pub fn RandomWalk() -> Html {
    let navigator = use_navigator().unwrap();
    let go_home = Callback::from(move |_| navigator.push(&Route::Home));
    let paint = Callback::from(move |_| {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        ctx.begin_path();

        // Draw the outer circle.
        ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        ctx.move_to(110.0, 75.0);
        ctx.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

        // Draw the left eye.
        ctx.move_to(65.0, 65.0);
        ctx.arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        ctx.move_to(95.0, 65.0);
        ctx.arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        ctx.stroke();
    });
    html! {
        <div>
            <button onclick={go_home}>{"Go Home"}</button>
            <button onclick={paint}>{"paint"}</button>
            <br />
            <canvas id="canvas" width="512px" height="512px" style="background: #aaa; margin: 20px;" />
        </div>
    }
}
