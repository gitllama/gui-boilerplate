// CLIを表示しない（アタッチされないので標準出力は出ない）
// #![windows_subsystem = "windows"]

use wry::{
  application::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
  },
  webview::WebViewBuilder,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Message{

  #[serde(rename = "type")]
  type_name: String,
  payload: String

}

#[derive(Debug)]
enum UserEvent {
  SendData(String),
}

fn main() -> wry::Result<()> {
  
  let html = include_str!("index.html");

  let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
  let proxy = event_loop.create_proxy();
  let window = WindowBuilder::new()
  .with_title("test")
  .build(&event_loop)?;

  let webview = WebViewBuilder::new(window)?
    .with_html(html)?
    // .with_url("http://tauri.app")?
    .with_ipc_handler(move |_window, arg| {
      let m = serde_json::from_str::<Message>(&arg).unwrap();
      println!("{m:?}");
      match m.type_name.as_str() {
        "test" => {

        },
        "test-2" => {
          let _ = proxy.send_event(UserEvent::SendData(m.payload)).unwrap();

        },
        _ => {},
      };
    })
    .with_devtools(true)
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("起動完了"),
      Event::UserEvent(UserEvent::SendData(e)) => {
        println!("{e:?}");
        let r = webview.evaluate_script(&*format!("console.log('{e}')"));
      }
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => {}
    }
  });
}
