use gloo_timers::callback::Interval;
use yew::{html, Component, Context, Html, Properties};

use super::grid::Grid;
use super::universe::Universe;

#[derive(PartialEq, Properties, Clone)]
pub struct GridSize {
    pub width: u32,
    pub height: u32,
}

#[derive(PartialEq, Properties, Clone)]
pub struct GameProps {
    pub started: bool,
    pub grid_size: GridSize,
}
pub struct Game {
    pub universe: Universe,
}

pub enum Msg {
    NextStep,
}

impl Component for Game {
    type Message = Msg;
    type Properties = GameProps;

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextStep => {
                self.universe.tick();
                true
            }
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        let universe = Universe::new(ctx.props().grid_size.width, ctx.props().grid_size.height);

        Self { universe }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let link = ctx.link().callback(|()| Msg::NextStep);

        if ctx.props().started {
            log::info!("started changed {}", ctx.props().started);
            let interval = Interval::new(300, move || {
                log::info!("next step");
                link.emit(());
            });

            interval.forget();
        };

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let onclick = link.callback(|_| Msg::NextStep);

        html! {
            <>
                <Grid grid={self.universe.clone()} />
                <button {onclick}>{ format!("Next step")}</button>
            </>
        }
    }
}
