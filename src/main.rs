use yew::prelude::*;

mod grid;
mod rule_bar;

use grid::Grid;
use rule_bar::{ColRuleBar, RowRuleBar};

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

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let row_rules = vec![vec![0], vec![4, 5, 6, 7, 8, 2], vec![0]];
        let col_rules = vec![vec![1], vec![1, 1, 1], vec![1]];
        html! {
            <div class="nonogram-container">
                <div class="grid-col-rules">
                    <ColRuleBar row_rules={row_rules.clone()} col_rules={col_rules.clone()}></ColRuleBar>
                </div>

                <div class="grid-row-rules">
                    <RowRuleBar row_rules={row_rules.clone()} col_rules={col_rules.clone()}></RowRuleBar>
                </div>
                <div class="grid-board">
                    <Grid rows=3 columns=3></Grid>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Nonogram>::new().render();
}
