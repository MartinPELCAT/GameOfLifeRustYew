pub mod components;

use components::game::Game;
use components::grid::GridProps;
use yew::prelude::*;

enum Msg {}

struct Model {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let grid_props = GridProps {
            width: 120,
            height: 80,
        };

        html! {
            <Game started={false} grid={grid_props} />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
