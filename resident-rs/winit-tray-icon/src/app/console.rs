#[cfg(target_os = "windows")]
use winapi::um::{
  consoleapi::AllocConsole, 
  wincon::{FreeConsole, GetConsoleWindow},
  // winuser::{ SetWindowLongW, GetWindowLongPtrW, GWL_EXSTYLE, WS_POPUP, WS_EX_APPWINDOW, ShowWindow, SW_HIDE, SW_SHOW }
};
// use winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO};

pub fn set_console(flag:bool) {
  #[cfg(target_os = "windows")]
  unsafe {
    let hwnd = GetConsoleWindow();
    if hwnd.is_null() { return; }
    if flag {
      FreeConsole();
    } else {
      AllocConsole();
    }
  }
}