use image::load_from_memory;
use tray_icon::{
  menu::{Menu, MenuItem, PredefinedMenuItem, /* MenuEvent, AboutMetadata,*/ },
  // TrayIcon, TrayIconBuilder, TrayIconEvent, TrayIconEventReceiver,
};

/******** icon ********/

const IMAGE: &[u8] = include_bytes!("../icon/icon.png");
const IMAGE_RED: &[u8] = include_bytes!("../icon/red.png");

// let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/tray/icon.png");
// let icon = icon::load_icon(std::path::Path::new(path));

pub fn load_icon(flag : bool) -> tray_icon::Icon {
  let (icon_rgba, icon_width, icon_height) = {
    let buffer = match flag {
      false => IMAGE,
      true => IMAGE_RED,
    };
    let image = load_from_memory(buffer).expect("Failed to load image").into_rgba8();
    // let image = image::open(path).expect("Failed to open icon path").into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };
  tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}


/******** tray (context menu) ********/


pub fn new_tray_menu() -> Menu {
  let menu = Menu::new();
  let console = MenuItem::with_id(crate::Action::ToggleConsole.to_string(), "Console", true, None);
  let separator = PredefinedMenuItem::separator();
  let quit_item = MenuItem::with_id(crate::Action::Quit.to_string(), "Exit", true, None);
  // let quit_item = PredefinedMenuItem::quit(None);

  if let Err(err) = menu.append(&console) { log::error!("{err:?}"); }
  menu.append_items(&[&separator]).unwrap();
  if let Err(err) = menu.append(&quit_item) { log::error!("{err:?}"); }
  menu
}

