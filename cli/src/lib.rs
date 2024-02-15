use backend::Backend;
use ds_lib::ClientKeyPackages;
use openmls::{key_packages::KeyPackageIn, prelude_test::KeyPackage};
use openmls_rust_crypto::OpenMlsRustCrypto;
use openmls_traits::OpenMlsCryptoProvider;
use tls_codec::TlsByteVecU8;
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
pub fn create_client(client_name: String) -> JsValue {
    let client = Some(user::User::new(client_name.clone()));
    serde_wasm_bindgen::to_value(&client).unwrap()
}

#[wasm_bindgen]
pub async fn register(client_value: JsValue) {
    let client = deserialize_client(client_value).unwrap();
    client.register().await;
}

#[wasm_bindgen]
pub fn create_kp(client_value: JsValue) -> JsValue {
    let client = deserialize_client(client_value).unwrap();
    let key_package = client.create_key_package();
    serde_wasm_bindgen::to_value(&key_package).unwrap()
}

#[wasm_bindgen]
pub async fn broadcast_kp(client_value: JsValue, kp_hash: Vec<u8>, kp_value: JsValue) {
    let client = deserialize_client(client_value).unwrap();
    let kp_data: KeyPackage = serde_wasm_bindgen::from_value(kp_value).unwrap();

    let kp = (kp_hash, kp_data);
    let ckp = ClientKeyPackages(
        vec![kp]
            .into_iter()
            .map(|(b, kp)| (b.into(), KeyPackageIn::from(kp)))
            .collect::<Vec<(TlsByteVecU8, KeyPackageIn)>>()
            .into(),
    );

    match (Backend::default()).publish_key_packages(&client, &ckp).await {
        Ok(()) => (),
        Err(e) => println!("Error sending new key package: {e:?}"),
    };
}

#[wasm_bindgen]
pub fn hash_kp(key_package_value: JsValue) -> JsValue {
    let key_package: KeyPackage = serde_wasm_bindgen::from_value(key_package_value).unwrap();
    let hash = key_package
        .hash_ref((OpenMlsRustCrypto::default()).crypto())
        .unwrap()
        .as_slice()
        .to_vec();
    serde_wasm_bindgen::to_value(&hash).unwrap()
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
pub async fn update(client_value: JsValue, group_id: Option<String>) -> JsValue {
    let mut client: user::User = match serde_wasm_bindgen::from_value(client_value) {
        Ok(c) => c,
        Err(e) => {
            panic!(" >>> Error updating client: {e}\n");
        }
    };

    let messages = match client.update(group_id).await {
        Ok(c) => c,
        Err(e) => {
            panic!(" >>> No updates: {e}\n");
        }
    };

    console::log_1(&format!(" >>> Updated client :)\n").into());

    if !messages.is_empty() {
        console::log_1(&format!("     New messages:\n\n").into());
    }
    messages.iter().for_each(|m| {
        console::log_1(&format!("         {m}\n").into());

    });
    console::log_1(&format!("\n").into());
    serde_wasm_bindgen::to_value(&messages).unwrap()
}

#[wasm_bindgen]
pub async fn reset() {
    backend::Backend::default().reset_server().await;
}
