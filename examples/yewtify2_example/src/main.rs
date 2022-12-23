use yew::prelude::*;
use yewtify2 as y2;

#[function_component]
fn App() -> Html {
    html! {
      <y2::App>
        <div>{ "I'm a child" }</div>
        <y2::Icon icon={y2::MdiIcon::Account} size={y2::composables::size::Size::Large}/>
      </y2::App>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}