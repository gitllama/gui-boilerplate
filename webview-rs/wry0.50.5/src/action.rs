#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "kind", content = "data")]
#[serde(rename_all = "camelCase")]
pub enum Action {
  #[serde(skip)]
  GET(wry::http::Request<Vec<u8>>),
  #[serde(skip)]
  POST(wry::http::Request<Vec<u8>>),
  Count(i32),
  Close,
  Unknown
}