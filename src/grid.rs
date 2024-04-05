use yew::prelude::*;

use crate::nonogram::SquareState;

pub struct GridComponent {
    pub props: GridProps,
}

#[derive(PartialEq, Properties, Clone)]
pub struct GridProps {
    pub rows: u32,
    pub columns: u32,
    pub values: Vec<SquareState>,
}

pub enum GridMessage {
    Toggle(usize),
}

impl Component for GridComponent {
    type Message = GridMessage;

    type Properties = GridProps;

    fn create(ctx: &Context<Self>) -> Self {
        GridComponent {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GridMessage::Toggle(i) => {
                self.props.values[i] = match self.props.values[i] {
                    SquareState::Filled => SquareState::Empty,
                    SquareState::Empty => SquareState::Unknown,
                    SquareState::Unknown => SquareState::Filled,
                };
            }
        }
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="nonogram-board">
            {
                for (0..self.props.rows).map(|row| { self.render_row(row, ctx) })
            }
            </div>
        }
    }
}

impl GridComponent {
    fn render_row(&self, row: u32, ctx: &Context<Self>) -> Html {
        html! {
            <div class="row">
                { for (0..self.props.columns).map(|col| self.render_square(row, col, ctx)) }
            </div>
        }
    }

    fn render_square(&self, row: u32, col: u32, ctx: &Context<Self>) -> Html {
        let index = (row * self.props.columns + col) as usize;

        let msg_click = ctx.link().callback(move |_| GridMessage::Toggle(index));
        
        match self.props.values[index] {
            SquareState::Filled => html! { <div class="square-filled" onclick={msg_click} /> },
            SquareState::Empty => html! { <div class="square-empty" onclick={msg_click}/> },
            SquareState::Unknown => html! { <div class="square-unknown" onclick={msg_click}/> },
        }
    }
}
