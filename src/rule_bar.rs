use std::task::Wake;

use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct RuleBarProps {
    pub row_rules: Vec<Vec<u32>>,
    pub col_rules: Vec<Vec<u32>>,
}

pub fn convert_rule_horizontal(rule: &[u32]) -> Html {
    rule.iter().map(|v| {
        let text = format!("{:} ", v);
        html! {text}
    }).collect()
}

pub fn convert_rule_vertical(rule: &[u32]) -> Html {
    rule.iter()
        .flat_map(|v| {
            [
                html! {
                   {v}
                },
                html! {
                    <br />
                },
            ]
            .into_iter()
        })
        .collect()
}

#[function_component]
pub fn RowRuleBar(props: &RuleBarProps) -> Html {
    html! {
    <div class="row-rules-container">
    {
    props.row_rules
        .iter()
        .map(|rule| {
            html! {
                <div class="row-rect">
                {convert_rule_horizontal(rule)}
                </div>
            }
        })
        .collect::<Html>()
    }
    </div>
    }
}

#[function_component]
pub fn ColRuleBar(props: &RuleBarProps) -> Html {
    // let col_rules = convert_rules_vertical(&props.col_rules);
    html! {
    <div class="col-rules-container">
        {props.col_rules.iter()
                .map(|rule| {
                    html! {
                        <div class="col-rect">
                        {convert_rule_vertical(rule)}
                        </div>
                    }
                })
                .collect::<Html>()
        }
    </div>
    }
}
