use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct RuleBarProps {
    pub rules: Vec<Vec<u32>>,
}

pub fn convert_rule_horizontal(rule: &[u32]) -> Html {
    rule.iter()
        .map(|v| {
            let text = format!("{:} ", v);
            html! {text}
        })
        .collect()
}

pub fn convert_rule_vertical(rule: &[u32], size: usize) -> Html {
    let diff = size - rule.len();
    (0..diff)
        .map(|_| {
            html! {<div> {"‎ "} </div>}
        })
        .chain(rule.iter().map(|v| {
            html! { <div> {v} </div> }
        }))
        .collect()
}

#[function_component]
pub fn RowRuleBar(props: &RuleBarProps) -> Html {
    html! {
    <div class="row-rules-container">
    {
    props.rules
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
    let size = props.rules.iter().map(Vec::len).max().unwrap();
    html! {
    <div class="col-rules-container">
        {props.rules.iter()
                .map(|rule| {
                    html! {
                        <div class="col-rect">
                            {convert_rule_vertical(rule, size)}
                        </div>
                    }
                })
                .collect::<Html>()
        }
    </div>
    }
}
