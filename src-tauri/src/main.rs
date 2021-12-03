#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  CustomMenuItem, 
  Menu, 
  Submenu
};

fn main() {

  let file = Submenu::new(
    "File",       
    Menu::new().add_item(CustomMenuItem::new("test", "Test").accelerator("CmdOrControl+T"))
  );

  let menu = Menu::new().add_submenu(file);

  tauri::Builder::default()
    .menu(menu)
    .on_menu_event(|event| {
      println!("{:?}", event.menu_item_id());
      match event.menu_item_id() {
          "test" => {
              println!("Test registered!");
          }
          _ => {}
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
