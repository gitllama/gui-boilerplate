mod app;
mod server;
mod err;

use clap::Parser;
use anyhow::Context as _;
use env_logger;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Parser, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[command(version)]
struct Args {
  #[arg(short = 'H', long, default_value_t = String::from("localhost:8000"))]
  host: String,

  #[arg(short, long)]
  current: Option<String>,
}

impl Args {
  
  fn init(&self) -> anyhow::Result<()> {
    let path = Self::set_current(self.current.clone())?;
    log::info!("working dir : {}", path.display());
    Ok(())
  }

  fn set_current(target : Option<String>) -> anyhow::Result<std::path::PathBuf> {
    if let Some(n) = target {
      let _ = std::env::set_current_dir(&n);
    }
    let path = std::env::current_dir().context("fail to get current_dir")?;
    Ok(path)
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Status {
  date : chrono::DateTime<chrono::Local>,
  options : Args
}

#[derive(Debug, strum::Display, serde::Deserialize)]
pub enum Action {
  #[serde(skip)]
  TrayIconEvent(tray_icon::TrayIconEvent),
  #[serde(skip)]
  MenuEvent(tray_icon::menu::MenuEvent),
  ToggleConsole,
  Quit,
  Notification,
  #[serde(skip)]
  Sleep(u64, tokio::sync::oneshot::Sender<String>),
  Unknown
}

fn main() -> anyhow::Result<()> {
  unsafe { std::env::set_var("RUST_LOG", "debug"); }

  // env_logger::init();
  let mut builder = env_logger::Builder::from_default_env();
  builder
    .format(formatter)
    .init();

  let args =Args::parse();
  let host = args.host.clone();
  args.init()?;

  let default_state = Status { 
    date : chrono::Local::now(),
    options : args
  };

  let shared_state = Arc::new(tokio::sync::Mutex::new(default_state));

  let event_loop = winit::event_loop::EventLoop::<Action>::with_user_event().build().unwrap();
  event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

  log::info!("booting server : {}", host);
  let proxy = Arc::new(tokio::sync::Mutex::new(event_loop.create_proxy()));
  let runtime = Runtime::new().expect("Failed to create Tokio runtime");
  let shared_state_axum = shared_state.clone();
  let proxy_clone = proxy.clone();
  runtime.spawn(async move {
    server::run_server(host.as_str(), shared_state_axum, proxy_clone).await;
  });  
  
  
  log::info!("booting app");
  let shared_state_axum = shared_state.clone();
  let proxy_clone = proxy.clone();
  let mut app = app::Application::new(shared_state_axum, proxy_clone);
  app::winit_init(&event_loop);

  // Since winit doesn't use gtk on Linux, and we need gtk for
  // the tray icon to show up, we need to spawn a thread
  // where we initialize gtk and create the tray_icon
  #[cfg(target_os = "linux")]
  std::thread::spawn(|| {
    gtk::init().unwrap();
    let _tray_icon = Application::new_tray_icon();
    gtk::main();
  });

  if let Err(err) = event_loop.run_app(&mut app) {
    log::error!("Error: {:?}", err);
  }

  Ok(())
}


fn formatter(buf: &mut env_logger::Formatter, record: &log::Record<'_>) -> Result<(), std::io::Error> {
  use std::io::Write;
  use env_logger::fmt::Color;
  if record.target().starts_with(module_path!()) {
    let mut level_style = buf.style();
    match record.level() {
      log::Level::Error => level_style.set_color(Color::Red).set_bold(true),
      log::Level::Warn => level_style.set_color(Color::Yellow).set_bold(true),
      log::Level::Info => level_style.set_color(Color::Green),
      log::Level::Debug => level_style.set_color(Color::Blue),
      log::Level::Trace => level_style.set_color(Color::Cyan),
    };
    let mut target_style = buf.style();
    target_style.set_color(Color::Ansi256(245));
    writeln!( buf, "{:<5} {} {}",
      level_style.value(record.level()),
      target_style.value(record.target()),
      record.args()
    )
  } else {
    write!(buf, "")
  }
}
