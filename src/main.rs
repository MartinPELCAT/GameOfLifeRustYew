pub mod components;
pub mod logic;

use components::game::{Game, GridSize};

use yew::prelude::*;

enum Msg {
    StartGame,
}

struct Model {
    started: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { started: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartGame => {
                if self.started {
                    return false;
                }
                self.started = true;
                log::info!("Game is started");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let grid_size = GridSize {
            width: 100,
            height: 100,
        };

        let link = ctx.link();

        html! {
            <>
                <Game started={self.started} grid_size={grid_size} />
                <button onclick={link.callback(|_| Msg::StartGame)}>{"Start the game"}</button>
                if self.started {
                    <div>{format!("Game is started")}</div>
                }

            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
