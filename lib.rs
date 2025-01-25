use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, Node, Window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str)
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    //get window object
    let window = web_sys::window().expect("no global 'window' exists");
    let document = window.document().expect("should hae a document on window");
    //fetch data from the server
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let request = web_sys::Request::new_with_str("/widgets")?;
        let _ = wasm_bindgen_futures::spawn_local(async move {
            match fetch_widgets(request).await {
                Ok(widgets) => {
                    let container = document
                                    .query_selector(".widgets")
                                    .expect("should have .widgets")
                                    .expect("Should succeed in finding the element");
                    for widget in widgets {
                        let widget_element = create_widget_element(&document, widget);
                        container.append_child(&widget_element)?;
                    }
                    resolve.call1(&JsValue::UNDEFINED, &JsValue::NULL)?;
                }
                Err(e) => {
                    log(&format!("Error fetching widgets: {:?}", e));
                    resolve.call1(&JsValue::UNDEFINED, &JsValue::NULL)?;
                }
            }
            Ok(())
        });
        Ok(())
    });
    wasm_bindgen_futures::spawn_local(async move {
        promise.await;
    });
    Ok(())
}

async fn fetch_widgets(request:web_sys::Request) -> Result<Vec<Widget>, JsValue> {
    let window = web_sys::window().expect("no global 'window' exists");
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;
    let json = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
    let widgets: Vec<Widget> = json.into_serde()?;
    Ok(widgets)
}

fn create_widget_element(document:&web_sys::Document, widget: Widger) -> Node {
    let widget_div = document.create_element("div").unwrap();
    widget_div.set_class_name("widget");

    let title = document.create_element("h2").unwrap();
    title.set_text_content(Some(&widget.title));

    let content = document.create_element("p").unwrap();
    content.set_text_content(Some(&widget.content));

    widget_div.append_child(&title).unwrap();
    widget_div.append_child(&content).unwrap();
    widget_div.into()
}

#[derive(serde::Deserialize)]
pub struct Widget {
    id: String,
    title: String,
    content: String,
}

// cargo install wasm-pack
//wasm-pack build --target web