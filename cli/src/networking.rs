use url::Url;

use tls_codec::Serialize;

use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use js_sys::wasm_bindgen::JsCast;

// TODO: return objects not bytes.

fn handle_jsvalue_error<T>(res: Result<T, JsValue>) -> Result<T, String> {
    match res {
        Ok(r) => return Ok(r),
        Err(e) => return Err(format!("Error: {e:?}")),
    };
}

pub async fn post(url: &Url, msg: &impl Serialize) -> Result<Vec<u8>, String> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let msg_vec = msg.tls_serialize_detached().unwrap();
    // let jv = JsValue::from(msg.tls_serialize_detached().unwrap());
    let u_copy;
    unsafe {
        let u = Uint8Array::view(&msg_vec);
        u_copy = u.slice(0, u.length())
    }
    let jv = JsValue::from(u_copy);

    opts.body(Some(&jv));

    // let request = Request::new_with_str_and_init(url.as_str(), &opts)?;
    let request;
    match Request::new_with_str_and_init(url.as_str(), &opts) {
        Ok(r) => {
            request = r;
        },
        Err(e) => return Err(format!("Error creating request: {e:?}")),
    };
    let window = web_sys::window().unwrap();
    let resp_value = handle_jsvalue_error(JsFuture::from(window.fetch_with_request(&request)).await)?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let blob = handle_jsvalue_error(resp.blob())?;
    let blob_jsvalue = handle_jsvalue_error(JsFuture::from(blob).await)?;

    let array = Uint8Array::new(&blob_jsvalue);
    return Ok(array.to_vec());
}

pub async fn get(url: &Url) -> Result<Vec<u8>, String> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request;
    match Request::new_with_str_and_init(url.as_str(), &opts) {
        Ok(r) => {
            request = r;
        },
        Err(e) => return Err(format!("Error creating request: {e:?}")),
    };
    // let request = Request::new_with_str_and_init(url.as_str(), &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = handle_jsvalue_error(JsFuture::from(window.fetch_with_request(&request)).await)?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let blob = handle_jsvalue_error(resp.blob())?;
    let blob_jsvalue = handle_jsvalue_error(JsFuture::from(blob).await)?;

    let array = Uint8Array::new(&blob_jsvalue);
    return Ok(array.to_vec());
}
