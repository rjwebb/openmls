use wasm_bindgen::prelude::*;

use web_sys::console;

extern crate console_error_panic_hook;
use std::panic;

mod backend;
mod conversation;
mod identity;
mod networking;
mod user;

pub fn deserialize_client(client_value: JsValue) -> Result<user::User, serde_wasm_bindgen::Error>{
    return serde_wasm_bindgen::from_value(client_value);
}

#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub async fn register(client_name: String) -> JsValue {
    let mut client  = Some(user::User::new(client_name.clone()));

    client.as_mut().unwrap().add_key_package();
    client.as_mut().unwrap().add_key_package();
    client.as_ref().unwrap().register().await;
    console::log_1(&format!("registered new client {}\n\n", client_name.clone()).into());

    return serde_wasm_bindgen::to_value(&client).unwrap();
}

#[wasm_bindgen]
pub async fn create_kp(client_value: JsValue) -> JsValue {
    let client = deserialize_client(client_value).unwrap();
    client.create_kp().await;

    console::log_1(&format!(" >>> New key package created\n\n").into());

    return serde_wasm_bindgen::to_value(&client).unwrap();
}

#[wasm_bindgen]
pub async fn create_group(client_value: JsValue, group_name: String) -> JsValue {
    let mut client = deserialize_client(client_value).unwrap();
    client.create_group(group_name.to_string());
    return serde_wasm_bindgen::to_value(&client).unwrap();
}

#[wasm_bindgen]
pub async fn send_msg(client_value: JsValue, msg: String, group_name: String) {
    let client = deserialize_client(client_value).unwrap();
    client.send_msg(&msg, group_name).await.unwrap();
}

#[wasm_bindgen]
pub async fn invite_client(client_value: JsValue, new_client: String, group_name: String) -> JsValue {
    let mut client = deserialize_client(client_value).unwrap();
    client.invite(new_client, group_name).await.unwrap();
    return serde_wasm_bindgen::to_value(&client).unwrap();
}

#[wasm_bindgen]
pub async fn remove_client(client_value: JsValue, rem_client: String, group_name: String) -> JsValue {
    let mut client = deserialize_client(client_value).unwrap();
    client.remove(rem_client.to_string(), group_name.to_string()).await.unwrap();
    return serde_wasm_bindgen::to_value(&client).unwrap();
}

#[wasm_bindgen]
pub async fn read_msgs(client_value: JsValue, group_name: String) -> JsValue {
    let client = deserialize_client(client_value).unwrap();
    let msgs = client.read_msgs(group_name).unwrap().unwrap();
    return serde_wasm_bindgen::to_value(&msgs).unwrap();
}


#[wasm_bindgen]
pub async fn update(client_value: JsValue, group_id: Option<String>) {
    let mut client: user::User = match serde_wasm_bindgen::from_value(client_value) {
        Ok(c) => c,
        Err(e) => {
            console::log_1(&format!(" >>> Error updating client: {e}\n").into());
            return;
        }
    };

    let messages = client.update(group_id).await.unwrap();
    console::log_1(&format!(" >>> Updated client :)\n").into());

    if !messages.is_empty() {
        console::log_1(&format!("     New messages:\n\n").into());
    }
    messages.iter().for_each(|m| {
        console::log_1(&format!("         {m}\n").into());

    });
    console::log_1(&format!("\n").into());
}

#[wasm_bindgen]
pub async fn reset() {
    backend::Backend::default().reset_server().await;
}
