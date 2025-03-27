use axum::{ extract::{Query, State}, response::Html, Json };
use serde_json::json;
use tera::{Tera, /* Context */};

const TEMPLATE_NAME : &str = "template.html";
const TEMPLATE : &str = indoc::indoc! {r#"
<!DOCTYPE html>
<html>
  <head>
    <title>{{name}} Page</title>
  </head>
  <body>
    <h1>Welcome {{name}}</h1>
  </body>
</html>
"#};

const TEMPLATE_CONFIG_NAME : &str = include_str!("config.html");
const TEMPLATE_CONFIG : &str = include_str!("./config.html");

#[allow(unused)]
pub async fn hello() -> &'static str { "hello world" }

pub async fn root() -> Html<String> {
  // let tera = Tera::new("templates/**/*").unwrap();
  let mut tera = Tera::default();
  tera.add_raw_template(TEMPLATE_NAME, TEMPLATE).expect("Failed to add template");

  let name = match std::env::current_exe() {
    Ok(exe_path) => {
      if let Some(program_name) = exe_path.file_name() {
        program_name.to_string_lossy().to_string()
      } else {
        "hoge".to_string()
      }
    }
    Err(e) => {
      println!("Error retrieving current executable path: {}", e);
      "".to_string()
    }
  };

  // let mut context = Context::new();
  let context = tera::Context::from_serialize(serde_json::json!({
    "name"  : name,
  })).unwrap();

  let rendered = tera.render(TEMPLATE_NAME, &context).expect("Failed to render template");
  Html(rendered)
}

pub async fn get(State(local_state): State<super::LocalState>) -> Json<crate::Status> {
  let dst = {
    let state = local_state.lock().await;
    state.clone()
  };
  Json(dst)
}

pub async fn set(State(local_state): State<super::LocalState>, Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
  log::info!("{:?}", payload);

  match serde_json::from_value(payload) {
    Ok(n) => {
    local_state.lock().await.options = n;
    log::info!("{:?}", local_state.lock().await.options);
    local_state.lock().await.options.init().unwrap();
    Json(json!({ "success" : true }))
    },
    Err(e) => {
      Json(json!({ "success" : false, "data" : e.to_string() }))
    }
  }

}

pub async fn config(State(local_state): State<super::LocalState>) -> Html<String> {
  use super::enum_util::ToTera;
  let mut tera = Tera::default();
  tera.add_raw_template(TEMPLATE_CONFIG_NAME, TEMPLATE_CONFIG).expect("Failed to add template");

  let data = local_state.lock().await.options.clone().to_tera();

  log::info!("{data:#?}");

  let context = tera::Context::from_serialize(serde_json::json!({
    "title"  : "CONFIG",
    "items" : data
  })).unwrap();

  let rendered = tera.render(TEMPLATE_CONFIG_NAME, &context).expect("Failed to render template");
  Html(rendered)
}


#[derive(serde::Deserialize)]
pub struct SleepParams { time: u64, }

pub async fn sleep(State(local_state): State<super::LocalState>, Query(params): Query<SleepParams>) -> String {
  let dst = local_state.sleep(params.time).await;
  log::debug!("handler : {}", dst);
  dst
}