use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
extern crate console_error_panic_hook;
mod elements;



#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_name = generatePDF)]
    fn generate_file(s: &[u8]);
}


#[wasm_bindgen]
pub async fn gen_pdf(json: &str) -> () {


    console_error_panic_hook::set_once();
    // Alert for debugging
    alert("Hello from gen_pdf function!");

    //let _pdf_byte =elements::gen_pdf(json);

    match elements::gen_pdf(json).await {
        Err(e) => {
            console_error(format!("Unable to list nodes: {:?}", e));
            println!();
            return;
        },
        Ok(byte)  => {
            generate_file(&byte);
            return;
        }
    }

}

// Function to log errors to the console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

fn console_error(s: String) {
    error(&s);
}