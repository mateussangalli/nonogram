use yew::prelude::*;

pub struct Nonogram {
    rows: u32,
    columns: u32,
}

impl Component for Nonogram {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Nonogram {
            rows: 10,
            columns: 10,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="nonogram-board">
            {
                for (0..self.rows).map(|row| { self.render_row(row) })
            }
            </div>
        }
    }
}

impl Nonogram {
    fn render_row(&self, row: u32) -> Html {
        html! {
            <div class="row">
                { for (0..self.columns).map(|col| self.render_square(row, col)) }
            </div>
        }
    }

    fn render_square(&self, row: u32, col: u32) -> Html {
        html! {
            <div class="square"></div>
        }
    }
}

fn main() {
    yew::Renderer::<Nonogram>::new().render();
}
