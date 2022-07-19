#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn rust_buttom_command() {
  println!("invoked from JS!");
}

#[tauri::command]
fn form_buttom_command() {
  println!("form invoked from JS!");
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
