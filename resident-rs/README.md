```bash
curl -i -X GET http://localhost:8000/ 
curl -i -X PUT http://localhost:8000/
curl -i -X OPTIONS http://localhost:8000/ # preflight request

# -X : GET以外のリクエスト(リクエストメソッドの指定)
# -v : 処理状況の更なる詳細や説明を出力させます。
# -i : HTTPヘッダを出力に含める
# -I : サーバーのレスポンスヘッダー表示
 
cargo run -- -c ../../browserExtensions_dpex/script   

```

```rust
/*
mpsc      : 送信者=複数 受信者= 1つ, キュー形式で順に処理
oneshot   : 送信者= 1つ 受信者= 1つ, 1回きりの送信, 送信が完了するとチャネルは閉じられる(非同期関数の結果返却など)
broadcast : 送信者=複数 受信者=複数, 古いメッセージを失う
watch     : 送信者= 1つ 受信者=複数, たくさんの値を送ることができるが履歴は残らない。受信側は最新の値のみを見ることができる。

Arc : スレッド安全
RwLock : 読み取りは並行に処理、書き込みは排他的に処理
Mutex  : 読み取り書き込み問わず排他的に処理
  tokio::sync::Mutex : 同期的 (blocking) なミューテックス
  std::sync::Mutex   : 非同期 (async) 対応のミューテックス
    lock : lock().await
    blocking_lock() : 同期関数 (sync) からでも使える
*/

use strum_macros::Display;
// strum::EnumStringだとパラメータ有が上手くいかないのでシリアライズで代用
#[derive(Debug, Display, Clone, Copy, strum::EnumString)]
pub enum UserMenuEvent {
}

/* 
だと二度手間でoneshot::channelでの戻り値も出来なさそうなので
直接EventLoopProxy参照に変更
*/
use tokio::sync::watch::Sender;
struct LocalState {
  tx : Arc<Sender<crate::UserEvent>>
}
fn hoge(){
  self.tx.send(crate::UserEvent::Notification).unwrap();
}

  // let (tx, rx) = tokio::sync::oneshot::channel();
  // let state_clone = self.state.clone();
  // runtime.spawn(async move {
  // match rx.await { }

    // set server event
    // let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    // let proxy = event_loop.create_proxy();
    // runtime.spawn(async move {
    //   loop {
        // let state = rx.borrow_and_update().clone();
        // let _ = proxy.send_event(state);
        // if rx.changed().await.is_err() { break; }
    //   }
    // });

```