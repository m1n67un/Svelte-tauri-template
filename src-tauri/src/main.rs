#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
  
fn main() {
  let context = tauri::generate_context!(); 
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(context)
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  println!("Hello, {}!", name);
  format!("Hello, {}!", name)
}