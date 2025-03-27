// #![windows_subsystem = "windows"] // Consoleの非表示

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
use nwd::NwgUi;
use nwg::NativeUi;

use once_cell::sync::OnceCell;
use async_std::task::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::process::{Command, Stdio};

use tide::prelude::json;

#[derive(Debug)]
pub struct UserData {
  busy : bool,
  count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Animal {
  data: String
}


static INSTANCE: OnceCell<Arc<Mutex<UserData>>> = OnceCell::new();


#[derive(Default, NwgUi)]
pub struct SystemTray {
  #[nwg_control]
  window: nwg::MessageWindow,

  #[nwg_resource(source_file: Some("./icon.ico"))]
  // #[nwg_resource(source_file: Some(include_bytes!("icon.ico")))]
  icon: nwg::Icon,

  #[nwg_control(icon: Some(&data.icon), tip: Some("shark 0.0.1"))]
  // #[nwg_control(icon: Some(&data.icon), tip: Some())]
  #[nwg_events(MousePressLeftUp: [SystemTray::show_menu], OnContextMenu: [SystemTray::show_menu])]
  tray: nwg::TrayNotification,

  #[nwg_control(parent: window, popup: true)]
  tray_menu: nwg::Menu,

  #[nwg_control(parent: tray_menu, text: "Add")]
  #[nwg_events(OnMenuItemSelected: [SystemTray::add])]
  tray_item1: nwg::MenuItem,

  #[nwg_control(parent: tray_menu, text: "Setting")]
  #[nwg_events(OnMenuItemSelected: [SystemTray::call_browser])]
  tray_item2: nwg::MenuItem,

  #[nwg_control(parent: tray_menu, text: "Exit")]
  #[nwg_events(OnMenuItemSelected: [SystemTray::exit])]
  tray_item3: nwg::MenuItem,

  #[nwg_control(parent: tray_menu, text: "Stop")]
  #[nwg_events(OnMenuItemSelected: [SystemTray::stop])]
  tray_item4: nwg::MenuItem,

}

impl SystemTray {

  fn show_menu(&self) {
    let (x, y) = nwg::GlobalCursor::position();
    self.tray_menu.popup(x, y);
  }

  fn exit(&self) { 
  
  
    nwg::stop_thread_dispatch(); 
  }

  fn call_browser(&self) {
    println!("call browser");
    
    let mut child = Command::new("powershell.exe")
    .args(["start", "http://localhost:3000/set"])
    .spawn()
    .expect("Failed to spawn child process");
  }


  fn hello3(&self) {
    println!("call");
    let buf = Arc::clone(INSTANCE.get().unwrap());

    let dst = web_view::builder()
    .title("Hello world!")
    .content(web_view::Content::Html(include_str!("webview.html")))
    .size(320, 240)
    .user_data( buf )
    .invoke_handler(|webview, arg| {
        if arg == "count" {
          Ok(())
          // webview.user_data_mut().count += 1;
          // webview.eval(&format!("update({})", webview.user_data().count))
        } else if arg == "drop" {
          Ok(())
          // println!("drop");
          // webview.eval(&format!("update({})", webview.user_data().count))
        } else {
          Ok(())
        }
    }).run();
    match dst {
      Ok(n) => {
        // println!("{:?}",n.count);
        // INSTANCE.set(UserData { count:n.count }).unwrap();
      },
      Err(n) => println!("{:?}", n),
    };
  }


  
  fn add(&self) {
    let buf = Arc::clone(INSTANCE.get().unwrap());

    let t = time::Duration::from_millis(1000);
    {
      let mut a = buf.lock().unwrap();
      (*a).busy = true;  
    }
    for _ in 0..10 {
      thread::sleep(t);
      let mut num = buf.lock().unwrap();
      (*num).count += 1;
      println!("count up : {:?}", num);
    }
    {
      let mut b = buf.lock().unwrap();
      (*b).busy = false;  
    }
  }

  fn stop(&self) {
    // task::block_on(async {
    //   let mut a = Fut.lock().unwrap().iter_mut();
    //   for f in a
    //   { 
    //     (*f).cancel().await;
    //   }
    // });

    // while fetch_handle.len() > 0 {
    //   let cur_thread = fetch_handle.remove(0); // moves it into cur_thread
    //   cur_thread.join().unwrap();
    // }
  }

}

use once_cell::sync::Lazy; // 1.3.1

static Fut: Lazy<Mutex<Vec<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(vec![]));

#[async_std::main]
async fn main() {
  
  // init
  let a = Arc::new(Mutex::new(UserData { 
    busy: false, 
    count: 0,
    // _exp: || _ui.hello3()
  }));
  INSTANCE.set(a).unwrap();
  println!("{:?}", INSTANCE);

  // set thread
  // let mut futures = vec![];

  Fut.lock().unwrap().push( task::spawn(async move {
    let mut app = tide::new();

    app.at("/").get(|_| async {
      let buf = Arc::clone(INSTANCE.get().unwrap());
      // let mut num = buf.lock().unwrap();
      // Ok(format!("Hello, world! {:?}", num)) 
      Ok(call(buf))
    });

    app.at("/set").get(|_| async {
      Ok(tide::Response::builder(200)
      .body(include_str!("main.html"))
      .header("Server", "tide")
      .content_type(tide::http::mime::HTML)
      .build())
    });

    app.at("/get").get(|_| async {
      Ok(json!([
          { "link": "123" },
          { "link": "456" }
      ]))
    });

    app.at("/post").post(|mut req: tide::Request<()>| async move {
      let animal: Animal = req.body_json().await?;
      println!("{:?}", animal);
      
      // let mut child = Command::new("powershell.exe")
      // .args(["dotnet", "script", "script.csx", format!("{}",animal.data).as_str()])
      // .spawn()
      // .expect("Failed to spawn child process");

      let mut child = Command::new("powershell.exe")
      .args(["Start-Process", "-FilePath", "dotnet", "-ArgumentList", format!("'script script.csx {}'",animal.data).as_str()])
      .spawn()
      .expect("Failed to spawn child process");

      Ok(tide::Body::from_json(&animal)?)
    });

    app.listen("127.0.0.1:3000").await.unwrap();
    println!("exit server");
  }));


  let task2 =  task::spawn(async move {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
    println!("exit ui");
    std::process::exit(0);
  });
  
  // println!("id = {}", task1.task().id());
  // println!("id = {}", task2.task().id());

  // futures.push(task1);
  // futures.push(task2);
  task2.await;
  // task::block_on(async { for f in futures { f.await } });

}


fn call(buf:Arc<Mutex<UserData>>) -> String {
  let mut data = buf.lock().unwrap();
  (*data).count += 1;
  if (*data).busy { return "Busy".to_string() }

  // data.busy = true;
  // let mut child = Command::new("dotnet.exe")
  //   .arg("script")
  //   .arg("script.csx")
  //   .stdin(Stdio::piped())
  //   .stdout(Stdio::piped())
  //   .spawn()
  //   .expect("Failed to spawn child process");

    let mut child = Command::new("Start-Process")
    .arg("-FilePath")
    .arg("dotnet.exe")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to spawn child process");


  format!("Hello, world! {:?}", data)
}
