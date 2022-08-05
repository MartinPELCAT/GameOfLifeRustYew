use std::{borrow::Borrow, ops::Deref, rc::Rc};

use gloo_timers::callback::Interval;
use yew::{function_component, html, use_effect_with_deps, use_reducer, Properties, Reducible};

use crate::logic::universe::Universe;

use super::grid::Grid;

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

enum GameAction {
    StartGame,
}

#[derive(Clone)]
struct GameState {
    universe: Universe,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            universe: Universe::new(100, 100),
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
    let universe_reducer = use_reducer(GameState::default);

    let started = props.started;
    let grid = universe_reducer.universe.borrow().deref().clone();

    {
        let universe = universe_reducer.clone();
        use_effect_with_deps(
            move |started| {
                let mut interval: Option<Interval> = None;

                if *started {
                    let tmp = Interval::new(200, move || {
                        universe.dispatch(GameAction::StartGame);
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
    }

    html! {
        <>
            <Grid grid={grid} />
        </>
    }
}
