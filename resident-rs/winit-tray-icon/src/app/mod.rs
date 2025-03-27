mod build_gui;
pub mod event;
mod console;

use tray_icon::{
  menu::{Menu, MenuEvent, /* AboutMetadata, MenuItem, PredefinedMenuItem */},
  TrayIcon, TrayIconBuilder, TrayIconEvent, /* TrayIconEventReceiver,*/
};
use winit::{
  application::ApplicationHandler,
  event_loop::{EventLoop, /* EventLoopBuilder, ControlFlow,*/ },
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::Action;

pub struct BackgroundWorker {
  runtime : tokio::runtime::Runtime,
  handle: Option<tokio::task::JoinHandle<()>>,
  // receiver: Option<tokio::sync::oneshot::Receiver<()>>,
  proxy: Arc<tokio::sync::Mutex<winit::event_loop::EventLoopProxy<crate::Action>>>
}
impl BackgroundWorker {

  pub fn new(proxy: Arc<tokio::sync::Mutex<winit::event_loop::EventLoopProxy<crate::Action>>>) -> Self {
    BackgroundWorker {
      runtime: tokio::runtime::Builder::new_multi_thread().enable_time().build().unwrap(),
      handle: None,
      proxy
    }
  }

  fn is_busy(&self) -> bool {
    if let Some(n) = &self.handle {
      !n.is_finished()
    } else {
      false
    }
  }

  fn run<F>(&mut self, unsync : bool, f : F) -> Option<String> where F: Future + Send + 'static, F::Output: Into<String> {
    match unsync {
      false => {
        println!("======== sync process start ========");
        let result = self.runtime.block_on(f);
        let dst : String = result.into();
        println!("======== process end ========");
        println!("result :");
        println!("{}", dst);
        Some(dst)
      },
      true => {
        println!("======== async process start ========");
        let proxy = self.proxy.clone();
        let handle = self.runtime.spawn(async move {
          let dst = f.await;
          println!("======== process end ========");
          println!("result :");
          println!("{}", dst.into());
          proxy.lock().await.send_event(crate::Action::Notification).unwrap();
        });
        self.handle = Some(handle);
        let proxy = self.proxy.clone();
        proxy.blocking_lock().send_event(crate::Action::Notification).unwrap();
        None
      }
    }

  }

}


pub struct Application {
  tray_icon: Option<TrayIcon>,
  console_visible : bool,
  bgw: BackgroundWorker,
  state : Arc<Mutex<crate::Status>>
}

impl Application {

  pub fn new(state : Arc<Mutex<crate::Status>>, proxy: Arc<tokio::sync::Mutex<winit::event_loop::EventLoopProxy<crate::Action>>>) -> Application { 
    Application { 
      tray_icon: None, 
      bgw : BackgroundWorker::new(proxy),
      console_visible : true,
      state
    } 
  }

  pub fn new_tray_icon() -> TrayIcon {

    TrayIconBuilder::new()
      .with_menu(Box::new(Self::new_tray_menu()))
      .with_menu_on_left_click(false)
      .with_tooltip("winit - awesome windowing lib")
      .with_icon(build_gui::load_icon(false))
      .with_title("x")
      .build()
      .unwrap()

  }

  pub fn new_tray_menu() -> Menu { build_gui::new_tray_menu() }

}

impl ApplicationHandler<Action> for Application {

  fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
    log::debug!("resumed");
  }

  fn window_event(
    &mut self,
    _event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    _event: winit::event::WindowEvent,
  ) {
    println!("window_event : {_event:?}")
  }

  fn new_events(
    &mut self,
    _event_loop: &winit::event_loop::ActiveEventLoop,
    cause: winit::event::StartCause,
  ) {
    // We create the icon once the event loop is actually running
    // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
    if winit::event::StartCause::Init == cause {
      #[cfg(not(target_os = "linux"))]
      { self.tray_icon = Some(Self::new_tray_icon()); }

      // We have to request a redraw here to have the icon actually show up.
      // Winit only exposes a redraw method on the Window so we use core-foundation directly.
      #[cfg(target_os = "macos")]
      unsafe {
        use objc2_core_foundation::{CFRunLoopGetMain, CFRunLoopWakeUp};
        let rl = CFRunLoopGetMain().unwrap();
        CFRunLoopWakeUp(&rl);
      }
    }
  }

  fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: Action) {
    event::user_event(self, event_loop, event);
  }

}

pub fn winit_init(event_loop: &EventLoop<Action>) {

  // set tray(mouse) event
  let proxy = event_loop.create_proxy();
  TrayIconEvent::set_event_handler(Some(move |event| { let _ = proxy.send_event(Action::TrayIconEvent(event));  }));
  
  // set menu event
  let proxy = event_loop.create_proxy();
  MenuEvent::set_event_handler(Some(move |event : tray_icon::menu::MenuEvent| { 
    // let e : crate::UserMenuEvent = id.parse().unwrap();
    // let _ = proxy.send_event(UserEvent::MenuEvent(event));
    let e: crate::Action = serde_json::from_str(format!("\"{}\"", event.id.0).as_str()).unwrap();
    let _ = proxy.send_event(e);
  }));

  let _menu_channel = MenuEvent::receiver();
  let _tray_channel = TrayIconEvent::receiver();

}

