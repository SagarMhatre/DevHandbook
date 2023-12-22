REf :https://yew.rs/docs/tutorial

```sh
cargo install trunk
rustup target add wasm32-unknown-unknown
cargo new yew-app
cd yew-app
```

Cargo.toml
```toml
[dependencies]
yew = { git = "https://github.com/yewstack/yew/", features = ["csr"] }
```

src/main.rs
```rs
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```
main.html
```html
<!doctype html>
<html lang="en">
    <head></head>
    <body></body>
</html>
```

trunk serve --open