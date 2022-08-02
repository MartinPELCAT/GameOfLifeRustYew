use yew::{html, Component, Context, Html, Properties};

use super::cell::Cell;

#[derive(PartialEq, Properties, Clone)]
pub struct GridProps {
    pub width: usize,
    pub height: usize,
}

pub struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Component for Grid {
    type Message = ();
    type Properties = GridProps;

    fn update(&mut self, _ctx: &Context<Self>, _msgg: Self::Message) -> bool {
        false
    }

    fn create(ctx: &Context<Self>) -> Self {
        // Create a grid of cells
        let mut grid = Vec::new();
        for _ in 0..ctx.props().height {
            let mut row = Vec::new();
            for _ in 0..ctx.props().width {
                row.push(false);
            }
            grid.push(row);
        }
        Self { grid }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            {self.grid.iter().map(|row| {
                html! {
                    <div style={"display:flex;"}>
                        {row.iter().map(|cell| {
                            html! {
                                <Cell is_alive={*cell} />
                            }
                        }).collect::<Html>()}
                    </div>
                }
            }).collect::<Html>()
        }
        }
    }
}
