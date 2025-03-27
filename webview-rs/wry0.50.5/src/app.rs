mod handler;



use crate::{ Action, State };
use handler::*;

use std::sync::{ Arc, Mutex };
// use dpi::{LogicalPosition, LogicalSize};
use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::{ActiveEventLoop, EventLoop},
  window::{ Window, WindowId },
};
use wry::{http::Method, WebViewBuilder};
// use wry::{http::Request, Rect, RequestAsyncResponder};
// use wry::http::header::CONTENT_TYPE;

#[derive(Default)]
pub struct App {
  start_url: String,
  window: Option<Window>,
  webview: Arc<Mutex<Option<wry::WebView>>>,
  state : Arc<Mutex<State>>
}

impl App {
  pub fn new(url: &str, state: State, _event_loop: &EventLoop<Action>) -> Self {
    Self {
      start_url : url.to_string(),
      window : None,
      webview : Arc::new(Mutex::new(None)),
      state : Arc::new(Mutex::new(state))
    }
  }
}

impl ApplicationHandler<Action> for App {

  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let state_clone_ipc = self.state.clone();
    let state_clone_protocol = self.state.clone();
    let wv = self.webview.clone();
  
    let window = event_loop.create_window(Window::default_attributes()).unwrap();
    let webview = WebViewBuilder::new()
      .with_asynchronous_custom_protocol("wry".into(), move | _webview_id, request, responder |{
        let state_clone3 = state_clone_protocol.clone();
        std::thread::spawn(move || {
          let method = request.method();
          let res = match *method {
            Method::GET => user_event (&state_clone3, None, Action::GET(request)),
            Method::POST => user_event (&state_clone3, None, Action::POST(request)),
            _ => user_event (&state_clone3, None, Action::Unknown),
          };
          let response = match res {
            Some(n) => n,
            None => wry::http::Response::builder().body(Vec::new()).unwrap()
          };
          responder.respond(response);
        });
      })
      .with_ipc_handler(move | req | {
        /* { method : GET, uri : "",  body : str } */
        match serde_json::from_str::<Action>(req.body()) {
         Ok(n) => user_event (&state_clone_ipc, None, n),
         Err(_e) => user_event (&state_clone_ipc, None, Action::Unknown),
        };
        let wv_lock = wv.lock().unwrap();
        let count = state_clone_ipc.lock().unwrap().count;
        let _ = wv_lock.as_ref().unwrap().evaluate_script(indoc::formatdoc!{r#"
          var customEvent = new CustomEvent('customEvent', {{
            'detail': {{
              'count' : {count}
            }}
          }});
          window.dispatchEvent(customEvent);
        "#}.as_str());
      })
      .with_url(self.start_url.as_str())
      .build(&window)
      .unwrap();

    self.window = Some(window);
    let mut wv = self.webview.lock().unwrap();
    *wv = Some(webview);

  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
    let state_clone = self.state.clone();
    match event {
      WindowEvent::CloseRequested => user_event(&state_clone, Some(event_loop), Action::Close),
      _ => user_event(&state_clone, Some(event_loop), Action::Unknown),
    };
  }

  fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
    use winit::event::StartCause;
    match cause {
      StartCause::Init => { },
      StartCause::Poll => { },
      StartCause::ResumeTimeReached { start:_s, requested_resume:_r } => { },
      StartCause::WaitCancelled { start:_s, requested_resume:_r } => { },
    };
  }

  fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Action) {
    println!("{event:?}");
    let state_clone = self.state.clone();
    user_event(&state_clone, Some(event_loop), event);
  }

}
