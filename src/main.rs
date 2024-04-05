use yew::prelude::*;
use serde_json::Result;
use serde::Deserialize;
use std::fs;

pub mod nonogram;
pub mod grid;
mod rule_bar;

use grid::GridComponent;
use rule_bar::{ColRuleBar, RowRuleBar};

struct Nonogram {
    props: NonogramProperties,
}

#[derive(Deserialize, PartialEq, Properties, Clone)]
struct NonogramProperties {
    rows: u32,
    cols: u32,
    col_rules: Vec<Vec<u32>>,
    row_rules: Vec<Vec<u32>>
}

impl Component for Nonogram {
    type Message = ();

    type Properties = NonogramProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Nonogram { props: ctx.props().clone() }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let values = vec![nonogram::SquareState::Unknown; (self.props.rows * self.props.cols) as usize];
        html! {
            <div class="nonogram-container">
                <div class="grid-col-rules">
                    <ColRuleBar rules={ctx.props().col_rules.clone()}></ColRuleBar>
                </div>

                <div class="grid-row-rules">
                    <RowRuleBar rules={ctx.props().row_rules.clone()}></RowRuleBar>
                </div>
                <div class="grid-board">
                    <GridComponent rows={ctx.props().rows} columns={ctx.props().cols} values={values}></GridComponent>
                </div>
            </div>
        }
    }
}

#[function_component]
fn App () -> Html {
    const JSON_DATA: &str = include_str!("../assets/question.json");
    // let data = fs::read_to_string("assets/question.json").expect("File not found.");

    let props: NonogramProperties = serde_json::from_str(JSON_DATA).expect("Could not parse JSON");

    html! {
        <Nonogram cols={props.cols} rows={props.rows} col_rules={props.col_rules} row_rules={props.row_rules}/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
