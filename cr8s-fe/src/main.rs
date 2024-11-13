use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <p>{"Olá Mundo!"}</p>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
