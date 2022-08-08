pub mod components;
pub mod logic;

use components::game::{Game, GridSize};
use yew::prelude::*;

#[function_component(Model)]
pub fn game_component() -> Html {
    let grid_size = use_state(|| GridSize {
        width: 100,
        height: 100,
    });

    let started = use_state(|| false);

    let on_start_click = {
        let started = started.clone();
        Callback::from(move |_| {
            started.set(true);
        })
    };

    html! {
        <>
            <Game started={*started} grid_size={*grid_size} />
            <button onclick={on_start_click}>{"Start the game"}</button>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
