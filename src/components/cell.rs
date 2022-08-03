use yew::{html, Component, Context, Html, Properties};

use super::universe::CellState;

#[derive(PartialEq, Properties)]
pub struct CellProps {
    pub state: CellState,
}

pub struct Cell;

impl Component for Cell {
    type Message = ();
    type Properties = CellProps;

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style={
                if ctx.props().state == CellState::Alive {
                    "background-color: yellow; width: 4px; height: 4px;"
                } else {
                    "background-color: gray; width: 4px; height: 4px;"
                }
            } />
        }
    }
}
