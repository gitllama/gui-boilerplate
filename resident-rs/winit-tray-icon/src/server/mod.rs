mod enum_util;
mod handler;

use axum::{ body::Body, http::{Request, StatusCode}, middleware, routing::{get, post}, Router };
use tower_http::cors::CorsLayer;
use std::sync::Arc;


#[derive(Clone)]
struct LocalState {
  state : Arc<tokio::sync::Mutex<crate::Status>>,
  proxy : Arc<tokio::sync::Mutex<winit::event_loop::EventLoopProxy<crate::Action>>>
}

impl LocalState {

  async fn lock(&self) -> tokio::sync::MutexGuard<'_, crate::Status> {
    self.state.lock().await
  }

  #[allow(unused)]
  async fn notify(&self){
    let proxy_lock = self.proxy.lock().await;
    proxy_lock.send_event(crate::Action::Notification).unwrap();
  }

  async fn sleep(&self, i:u64) -> String {
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    {
      let proxy_lock = self.proxy.lock().await;
      proxy_lock.send_event(crate::Action::Sleep(i, tx)).unwrap();  
    }
    match rx.await {
      Ok(res) => format!("success : {}", res),
      Err(_) => "Error receiving response".to_string(),
    }
  }
  
}

#[allow(unused)]
async fn access_log_on_request( req: Request<Body>, next: middleware::Next, ) -> Result<axum::response::Response, StatusCode> {
  let origin = req.headers().get("Origin").and_then(|h| h.to_str().ok());
  if let Some(origin) = origin {
    if !origin.starts_with("chrome-extension://") || origin.is_empty() {
      log::error!("Blocked request from origin: {}", origin);
      return Err(StatusCode::FORBIDDEN);
    }
  }
  Ok(next.run(req).await)
}
async fn access_log( req: Request<Body>, next: middleware::Next, ) -> Result<axum::response::Response, StatusCode> {
  log::debug!("access_log: {:#?}", req);
  Ok(next.run(req).await)
}

pub async fn run_server(addr:&str , state: Arc<tokio::sync::Mutex<crate::Status>>, proxy : Arc<tokio::sync::Mutex<winit::event_loop::EventLoopProxy<crate::Action>>>) {

  let main_router = Router::new()
    .route("/sleep", get(handler::sleep))
    .route("/get", get(handler::get))
    .route("/set", post(handler::set))
    .route("/config", get(handler::config));

  let config_router = Router::new()
    .route("/", get(handler::root))
    .route("/", post(handler::hello))
    .layer( CorsLayer::new()
      // .allow_methods(vec![axum::http::Method::POST])
      .allow_methods(tower_http::cors::Any)
      .allow_headers(vec![axum::http::header::CONTENT_TYPE]) 
    );
 
  let app = Router::new()
    .merge(main_router)
    .merge(config_router)
    .layer( tower::ServiceBuilder::new().layer(axum::middleware::from_fn(access_log)) )
    .with_state(LocalState{ state, proxy });

  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}