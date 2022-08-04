use yew::{function_component, html, Properties};

use crate::logic::universe::CellState;

#[derive(PartialEq, Properties)]
pub struct CellProps {
    pub state: CellState,
}

#[function_component(Cell)]
pub fn chat_container(props: &CellProps) -> Html {
    html! {
        <div style={
            if props.state == CellState::Alive {
                "background-color: yellow; width: 4px; height: 4px;"
            } else {
                "background-color: gray; width: 4px; height: 4px;"
            }
        } />
    }
}
