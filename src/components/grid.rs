use crate::logic::universe::Universe;

use super::cell::Cell;
use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct GridProps {
    pub grid: Universe,
}

#[function_component(Grid)]
pub fn grid_component(props: &GridProps) -> Html {
    let grid = props.grid.get_grid();

    html! {
        {grid.iter().map(|row| {
            html! {
                <div style={"display:flex;"}>
                   {row.iter().map(|cell| {
                           html! {
                               <Cell state={*cell} />
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        }).collect::<Html>()}
    }
}
