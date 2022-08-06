use std::{borrow::Borrow, ops::Deref, rc::Rc};

use gloo_timers::callback::Interval;
use yew::{function_component, html, use_effect_with_deps, use_reducer, Properties, Reducible};

use crate::logic::universe::Universe;

use super::grid::Grid;

#[derive(PartialEq, Properties, Clone, Copy)]
pub struct GridSize {
    pub width: u32,
    pub height: u32,
}

#[derive(PartialEq, Properties, Clone)]
pub struct GameProps {
    pub started: bool,
    pub grid_size: GridSize,
}

enum GameAction {
    StartGame,
}

#[derive(Clone)]
struct GameState {
    universe: Universe,
}

impl GameState {
    fn init(width: u32, height: u32) -> Self {
        Self {
            universe: Universe::new(width, height),
        }
    }
}

impl Reducible for GameState {
    type Action = GameAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_universer: Universe = match action {
            GameAction::StartGame => {
                let mut universe = self.universe.clone();
                universe.next_step();
                universe
            }
        };

        Self {
            universe: next_universer,
        }
        .into()
    }
}

#[function_component(Game)]
pub fn game_component(props: &GameProps) -> Html {
    let universe_reducer =
        use_reducer(|| GameState::init(props.grid_size.width, props.grid_size.height));

    let started = props.started;
    let grid = universe_reducer.universe.borrow().deref().clone();

    use_effect_with_deps(
        move |started| {
            let mut interval: Option<Interval> = None;

            if *started {
                let tmp = Interval::new(100, move || {
                    universe_reducer.dispatch(GameAction::StartGame);
                });

                interval = Some(tmp);
            };
            || {
                if let Some(interval) = interval {
                    interval.cancel();
                }
            }
        },
        started,
    );

    html! {
        <>
            <Grid grid={grid} />
        </>
    }
}
