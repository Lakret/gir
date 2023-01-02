// use wasm_bindgen::prelude::*;
use yew::prelude::*;

// struct Model {
//   link: ComponentLink<Self>,
//   value: i64,
// }

// enum Msg {
//   AddOne,
// }

// impl Component for Model {
//   type Message = Msg;
//   type Properties = ();

//   fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//     Self { link, value: 0 }
//   }

//   fn update(&mut self, msg: Self::Message) -> ShouldRender {
//     match msg {
//       Msg::AddOne => self.value += 1,
//     }

//     true
//   }

//   fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//     // Should only return "true" if new properties are different to
//     // previously received properties.
//     // This component has no properties so we will always return "false".
//     false
//   }

//   fn view(&self) -> Html {
//     html! {
//       <div>
//         <div class="row">
//           <input class="cell"/>
//           <input class="cell"/>
//           <input class="cell"/>
//         </div>

//         <div class="row">
//           <input class="cell"/>
//           <input class="cell"/>
//           <input class="cell"/>
//         </div>
//       </div>

//         // <div>
//         //     <button onclick=self.link.callback(|_| Msg::AddOne) class="btn">{ "+1" }</button>
//         //     <p>{ self.value }</p>
//         // </div>
//     }
//   }
// }

// #[wasm_bindgen(start)]
// pub fn run_app() {
//   App::<Model>::new().mount_to_body();
// }

#[function_component]
fn App() -> Html {
  let counter = use_state(|| 0);
  let onclick = {
    let counter = counter.clone();
    move |_| {
      let value = *counter + 1;
      counter.set(value);
    }
  };

  html! {
      <div>
          <button {onclick}>{ "+1" }</button>
          <p>{ *counter }</p>
      </div>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
