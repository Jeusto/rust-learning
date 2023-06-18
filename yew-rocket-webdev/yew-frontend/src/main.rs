use gloo::console;
use yew::{html, Component, Context, Html};

pub enum Msg {
    Increment,
    Decrement,
    Reset,
}

pub struct App {
    counter_value: i32,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { counter_value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.counter_value += 1;
                console::log!("plus one");
                true // return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.counter_value -= 1;
                console::log!("minus one");
                true
            }
            Msg::Reset => {
                self.counter_value = 0;
                console::log!("reset");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="panel">
                    <button
                        onclick={ctx.link().callback(|_| Msg::Increment)}
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                        { "Increment" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "Decrement" }
                    </button>
                    <button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
                        { "Increment Twice" }
                        { "Reset" }
                    </button>
                </div>
                <p class="counter">
                    { self.counter_value }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
