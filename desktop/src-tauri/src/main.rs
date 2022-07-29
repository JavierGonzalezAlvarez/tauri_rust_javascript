#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Serialize, Debug, Deserialize)]
struct Employee {
  //_id: i32,
  name: String,
  phone: String,
  email: String,
  comments: String,
}

#[command]
fn rust_buttom_command() {
  println!("invoked from JS!");
}

#[command]
fn form_buttom_command(data: String) -> Result<(), ()> {
  println!("data coming from JS: {}", data);

  //deserializes
  let valor: Employee = serde_json::from_str(&data).unwrap();
  println!("email get from the front: {}", &valor.email);
  println!("name to add to the DataBase: {}", valor.name);

  let name: String = valor.name;
  let phone: String = valor.phone;
  let email: String = valor.email;
  let comments: String = valor.comments;

  let mut client =
    Client::connect("postgresql://test:2525_ap@localhost:5432/rustyew", NoTls).unwrap();

  client
    .execute(
      "INSERT INTO tbemployee (name, phone, email, comments) VALUES ($1, $2, $3, $4)",
      &[&name, &phone, &email, &comments],
    )
    .unwrap();

  Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      form_buttom_command,
      rust_buttom_command
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
