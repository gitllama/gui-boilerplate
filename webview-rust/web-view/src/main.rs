use web_view::*;

fn main() {
  let html_content = "<html><body><h1>Hello, World!</h1></body></html>";

  web_view::builder()
    .title("My Project")
    .content(Content::Html(html_content))
    .size(320, 480)
    .resizable(false)
    .debug(true)
    .user_data(())
    .invoke_handler(|webview, arg| {
      match arg {
        "test_one" => {
          Ok(())
        }
        "test_two" => {
          // Invoke a JavaScript function!
          webview.eval(&format!("myFunction({}, {})", 123, 456))
        }
        _ => Ok(()),
      }
    })
    .run()
    .unwrap();
}