use yew::{html,  Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct CellProps {
    pub is_alive: bool,
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
                if ctx.props().is_alive {
                    "background-color: black; width: 10px; height: 10px;"
                } else {
                    "background-color: gray; width: 10px; height: 10px;"
                }
            } />
        }
    }
}
