// CLIを表示しない（アタッチされないので標準出力は出ない）
// #![windows_subsystem = "windows"]

use tao::{
  event::{Event, StartCause, WindowEvent},
  event_loop::{ControlFlow, /* EventLoop */ EventLoopBuilder},
  window::WindowBuilder,
};
use wry::{webview_version, WebViewBuilder};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Message{

  #[serde(rename = "type")]
  type_name: String,
  payload: String

}

#[derive(Debug)]
pub enum UserEvent {
  SendData(String),
}

static RESOURCE: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/src/resource");

fn main() -> wry::Result<()> {

  let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
  let proxy_req = Arc::new(Mutex::new(event_loop.create_proxy()));

  let window = WindowBuilder::new()
    .with_title("Hello World")
    .build(&event_loop)
    .unwrap();
  let webview = WebViewBuilder::new(&window)
    .with_url("http://wry.localhost/resource/index.html")
    .with_asynchronous_custom_protocol("wry".into(), move |request, responder| {
      let proxy_req_clone = proxy_req.clone();
      
      std::thread::spawn(move || {
        println!("custom protocol resource path {:?}", request.uri().path());
        let url = request.uri().path().trim_start_matches("/resource/");
        let content = match RESOURCE.get_file(url){
          Some(n) => n.contents_utf8().unwrap(),
          None => {
            println!("{} {}", "get_file err", url);
            ""
          }
        };

        // std::thread::sleep(std::time::Duration::from_secs(2));
        // let _ = proxy_req_clone.lock().unwrap().send_event(UserEvent::SendData("1".to_string())).unwrap();
        // std::thread::sleep(std::time::Duration::from_secs(2));
        // let _ = proxy_req_clone.lock().unwrap().send_event(UserEvent::SendData("2".to_string())).unwrap();

        let header = match url {
          n if n.ends_with(".js") => "text/javascript",
          n if n.ends_with(".html") => "text/html",
          n if n.ends_with(".css") => "text/css",
          _=> "text/plain"
        };
        
        responder.respond(wry::http::Response::builder()
          .header(wry::http::header::CONTENT_TYPE, header)
          .body(content.to_string().as_bytes().to_vec())
          .unwrap());
      });
    })
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
      Event::UserEvent(UserEvent::SendData(e)) => {
        let _ = webview.evaluate_script(&*format!("console.log('{e}')"));
      },
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => (),
    }
  });

}
