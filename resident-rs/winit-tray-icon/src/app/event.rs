use super::console::set_console;
use crate::Action;

#[allow(unused_variables)]
pub fn user_event(app : &mut super::Application, event_loop: &winit::event_loop::ActiveEventLoop, event: Action) {
  use Action::*;
  use tray_icon::TrayIconEvent::{Click, DoubleClick};
  match event {
    TrayIconEvent(Click { id, position, rect, button, button_state }) => {
      return;
    },
    TrayIconEvent(DoubleClick { id, position, rect, button}) => {
      log::debug!("DoubleClick : {button:?}");
      let host = app.state.blocking_lock().options.host.clone();
      if webbrowser::open(format!("http://{host}").as_str()).is_ok() {
        log::info!("opened the browser : {}", host);
      } else {
        log::error!("Failed to open the browser : {}", host);
      }
    },
    TrayIconEvent(_) => { },
    ToggleConsole => {
      set_console(app.console_visible);
      app.console_visible = !app.console_visible;
    },
    Notification => {
      if let Some(icon) = app.tray_icon.as_mut() {
        let data = super::build_gui::load_icon(app.bgw.is_busy());
        let _ = icon.set_icon(Some(data));
      }
    },
    Sleep(time, tx) => {
      let res = if app.bgw.is_busy() {
        "isBusy".to_string()
      } else {
        let is_unsync = time > 3;
        let dst = app.bgw.run(is_unsync, async move {
          for i in 1..=time {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("{}", i);
          }
          "finished".to_string()
        });
        format!("started {dst:?}")
      };
      tx.send(format!("sleep process : {res}")).unwrap();
    },
    Quit => { /*self.quit_requested = true;*/ event_loop.exit(); },
    _ =>{ log::error!("Unknown Action"); }
  }
}
