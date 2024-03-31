use yew::prelude::*;

mod grid;
mod rule_bar;

use grid::Grid;
use rule_bar::RowRuleBar;

struct Nonogram {
    rows: u32,
    cols: u32,
}

impl Component for Nonogram {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Nonogram { rows: 10, cols: 10 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let rules = vec![vec![0], vec![1, 2], vec![0]];
        html! {
            <div class="nonogram-container">
                <div>
                    <RowRuleBar rules={rules}></RowRuleBar>
                </div>
                <div>
                    <Grid rows=3 columns=3></Grid>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Nonogram>::new().render();
}
