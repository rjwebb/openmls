// #[macro_use]
// extern crate clap;
// use clap::App;

use std::io::{stdin, stdout, StdoutLock, Write};
use wasm_bindgen::prelude::*;

use web_sys::console;

extern crate console_error_panic_hook;
use std::panic;

mod backend;
mod conversation;
mod identity;
mod networking;
mod user;


const HELP: &str = "
>>> Available commands:
>>>     - update                                update the client state
>>>     - reset                                 reset the server
>>>     - register {client name}                register a new client
>>>     - create kp                             create a new key package
>>>     - create group {group name}             create a new group
>>>     - group {group name}                    group operations
>>>         - send {message}                    send message to group
>>>         - invite {client name}              invite a user to the group
>>>         - read                              read messages sent to the group (max 100)
>>>         - update                            update the client state

";


// not sure about the lifetime here
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
pub async fn register(client_name: String) -> JsValue {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut client  = Some(user::User::new(client_name.clone()));

    client.as_mut().unwrap().add_key_package();
    client.as_mut().unwrap().add_key_package();
    client.as_ref().unwrap().register().await;
    console::log_1(&format!("registered new client {}\n\n", client_name.clone()).into());

    return serde_wasm_bindgen::to_value(&client).unwrap();
}

// pub async fn create_kp(client: &mut user::User) -> JsValue {
//     panic::set_hook(Box::new(console_error_panic_hook::hook));

//     client.as_mut().unwrap().create_kp();
//     console::log_1(&format!(" >>> New key package created\n\n").into());

//     return serde_wasm_bindgen::to_value(&client).unwrap();
// }

/*
fn main() {
    // Create a new group.
    if let Some(group_name) = op.strip_prefix("create group ") {
        if let Some(client) = &mut client {
            client.create_group(group_name.to_string());
            stdout
                .write_all(format!(" >>> Created group {group_name} :)\n\n").as_bytes())
                .unwrap();
        } else {
            stdout
                .write_all(b" >>> No client to create a group :(\n\n")
                .unwrap();
        }
        continue;
    }

    // Group operations.
    if let Some(group_name) = op.strip_prefix("group ") {
        if let Some(client) = &mut client {
            loop {
                stdout.write_all(b" > ").unwrap();
                stdout.flush().unwrap();
                let op2 = stdin.read_line().unwrap().unwrap();

                // Send a message to the group.
                if let Some(msg) = op2.strip_prefix("send ") {
                    match client.send_msg(msg, group_name.to_string()) {
                        Ok(()) => stdout
                            .write_all(format!("sent message to {group_name}\n\n").as_bytes())
                            .unwrap(),
                        Err(e) => println!("Error sending group message: {e:?}"),
                    }
                    continue;
                }

                // Invite a client to the group.
                if let Some(new_client) = op2.strip_prefix("invite ") {
                    client
                        .invite(new_client.to_string(), group_name.to_string())
                        .unwrap();
                    stdout
                        .write_all(
                            format!("added {new_client} to group {group_name}\n\n").as_bytes(),
                        )
                        .unwrap();
                    continue;
                }

                // Remove a client from the group.
                if let Some(rem_client) = op2.strip_prefix("remove ") {
                    client
                        .remove(rem_client.to_string(), group_name.to_string())
                        .unwrap();
                    stdout
                        .write_all(
                            format!("Removed {rem_client} from group {group_name}\n\n")
                                .as_bytes(),
                        )
                        .unwrap();
                    continue;
                }

                // Read messages sent to the group.
                if op2 == "read" {
                    let messages = client.read_msgs(group_name.to_string()).unwrap();
                    if let Some(messages) = messages {
                        stdout
                            .write_all(
                                format!(
                                    "{} has received {} messages\n\n",
                                    group_name,
                                    messages.len()
                                )
                                .as_bytes(),
                            )
                            .unwrap();
                    } else {
                        stdout
                            .write_all(format!("{group_name} has no messages\n\n").as_bytes())
                            .unwrap();
                    }
                    continue;
                }

                // Update the client state.
                if op2 == "update" {
                    update(client, Some(group_name.to_string()), &mut stdout);
                    continue;
                }

                // Exit group.
                if op2 == "exit" {
                    stdout.write_all(b" >>> Leaving group \n\n").unwrap();
                    break;
                }

                stdout
                    .write_all(b" >>> Unknown group command :(\n\n")
                    .unwrap();
            }
        } else {
            stdout.write_all(b" >>> No client :(\n\n").unwrap();
        }
        continue;
    }

    // Update the client state.
    if op == "update" {
        if let Some(client) = &mut client {
            update(client, None, &mut stdout);
        } else {
            stdout
                .write_all(b" >>> No client to update :(\n\n")
                .unwrap();
        }
        continue;
    }

    // Reset the server and client.
    if op == "reset" {
        backend::Backend::default().reset_server();
        client = None;
        stdout.write_all(b" >>> Reset server :)\n\n").unwrap();
        continue;
    }

    // Print help
    if op == "help" {
        stdout.write_all(HELP.as_bytes()).unwrap();
        continue;
    }

    stdout
        .write_all(b" >>> unknown command :(\n >>> try help\n\n")
        .unwrap();

}
 */
