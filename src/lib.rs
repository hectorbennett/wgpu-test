use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello world!");
}

fn init_canvas() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    canvas.set_width(500);
    canvas.set_height(500);
    body.append_child(&canvas)?;

    Ok(())
}

#[wasm_bindgen]
pub fn render_canvas() -> Result<(), JsValue> {
    init_canvas()?;
    Ok(())
}
