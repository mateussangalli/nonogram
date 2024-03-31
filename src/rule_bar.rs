use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct RuleBarProps {
    pub rules: Vec<Vec<u32>>,
}

fn rule_to_string(rule: &[u32]) -> String {
    rule.iter().map(|v| {format!("{:} ", v)}).collect()
}

#[function_component]
pub fn RowRuleBar(props: &RuleBarProps) -> Html {
    props.rules.iter()
        .map(|rule| {
            html! {
                <div class="row-rect">
                {rule_to_string(rule)}
                </div>
            }
        })
        .collect()
}
