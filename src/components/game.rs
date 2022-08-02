use yew::{html, Component, Context, Html, Properties};

use super::grid::{Grid, GridProps};

#[derive(PartialEq, Properties, Clone)]
pub struct GameProps {
    pub started: bool,
    pub grid: GridProps,
}

pub struct Game;

impl Component for Game {
    type Message = ();
    type Properties = GameProps;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let grid = ctx.props().clone().grid;
        html! {
            <Grid ..grid />
        }
    }
}
