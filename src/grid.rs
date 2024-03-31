use yew::prelude::*;

pub struct Grid {
    pub props: GridProps,
}

#[derive(PartialEq, Properties, Clone)]
pub struct GridProps {
    pub rows: u32,
    pub columns: u32,
}

impl Component for Grid {
    type Message = ();

    type Properties = GridProps;

    fn create(ctx: &Context<Self>) -> Self {
        Grid {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="nonogram-board">
            {
                for (0..self.props.rows).map(|row| { self.render_row(row) })
            }
            </div>
        }
    }
}

impl Grid {
    fn render_row(&self, row: u32) -> Html {
        html! {
            <div class="row">
                { for (0..self.props.columns).map(|col| self.render_square(row, col)) }
            </div>
        }
    }

    fn render_square(&self, row: u32, col: u32) -> Html {
        html! {
            <div class="square"></div>
        }
    }
}
