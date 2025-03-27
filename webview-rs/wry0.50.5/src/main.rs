// CLIを表示しない（アタッチされないので標準出力は出ない）
// #![windows_subsystem = "windows"]
mod action;
mod state;
mod app;

use action::*;
use state::*;

static RESOURCE: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/src/resource");

fn main() -> anyhow::Result<()> {

  let state = State::default();

  // let event_loop = winit::EventLoop::new().unwrap();
  // let mut app = App::default();

  let event_loop = winit::event_loop::EventLoop::<Action>::with_user_event().build()?;
  let mut app = app::App::new("http://wry.localhost/resource/index.html", state, &event_loop);
  event_loop.run_app(&mut app).unwrap();
  
  Ok(())
}
