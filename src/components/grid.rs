use super::{
    cell::Cell,
    universe::{CellState, Universe},
};
use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct GridProps {
    pub grid: Universe,
}

pub struct Grid {}

impl Component for Grid {
    type Message = ();
    type Properties = GridProps;

    fn update(&mut self, _ctx: &Context<Self>, _msgg: Self::Message) -> bool {
        false
    }

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let grid = ctx.props().grid.cells.clone();
        let width = usize::try_from(ctx.props().grid.width).unwrap();
        let height = usize::try_from(ctx.props().grid.height).unwrap();

        let mut rows: Vec<Vec<CellState>> = Vec::new();
        for y in 0..height {
            let mut row: Vec<CellState> = Vec::new();
            for x in 0..width {
                let cell = grid[y * width + x];
                row.push(cell);
            }
            rows.push(row);
        }

        html! {
            rows.iter().map(|row| {
                html! {
                    <div style={"display:flex;"}>
                        {row.iter().map(|cell| {
                            html! {
                                <Cell state={*cell} />
                            }
                        }).collect::<Html>()}
                    </div>
                }
            }).collect::<Html>()
        }
    }
}
