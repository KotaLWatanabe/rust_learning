// ============================================================
// Rust学習 フェーズ8: async/await・Tokio
// ============================================================
// 実行方法:
//   cargo test --test phase8_async          # このファイル全体
//   cargo test --test phase8_async test_r01 # 復習だけ
//   cargo test --test phase8_async test_q01 # 新問Q1だけ
//
// 事前準備 (Cargo.toml に追加が必要):
//   [dependencies]
//   tokio = { version = "1", features = ["full"] }
//
//   [dev-dependencies]
//   tokio = { version = "1", features = ["full"] }
//
// Scalaとの対応:
//   async fn        ≒ def f(): Future[T] (cats-effect の IO に近い)
//   .await          ≒ .unsafeRunSync() / Await.result()
//   tokio::spawn    ≒ Future { ... } / IO.start
//   tokio::select!  ≒ IO.race
//   tokio::join!    ≒ IO.both / (fa, fb).parMapN
//   Channel         ≒ Queue / MVar
//   Mutex           ≒ Ref (cats-effect) / synchronized
//
// ★ Rustのasyncの仕組み
//   async fn は Future トレイトを返す「状態機械」に変換される。
//   Future は「実行時」(executor) がないと動かない。
//   Tokio がその executor (= スレッドプール + イベントループ)。
//   .await は「この Future が完了するまで現在のタスクを一時停止」する。
// ============================================================

use std::time::Duration;
use tokio::time::sleep;

// ============================================================
// 【フェーズ7 復習】
// ============================================================

// R01: モジュールと可視性
// 【問題】以下の converter モジュールを実装してください。
// - celsius_to_fahrenheit: pub
// - fahrenheit_to_celsius: pub
// - round2: 小数点2桁に丸める内部ヘルパー (pub にしない)

mod converter {
    pub fn celsius_to_fahrenheit(c: f64) -> f64 {
        round2(c * 9.0 / 5.0) + 32.0
    }
    pub fn fahrenheit_to_celsius(f: f64) -> f64 {
        round2((f - 32.0) * 5.0 / 9.0)
    }
    fn round2(v: f64) -> f64 {
        (v * 100.0).round() / 100.0
    }
}

// R02: newtype パターン
// 【問題】Meters と Feet の newtype を作り、相互変換を実装してください。
// 1 foot = 0.3048 meters

struct Meters(f64);
struct Feet(f64);

impl Meters {
    fn new(v: f64) -> Meters {
        Meters(v)
    }
    fn value(&self) -> f64 {
        self.0
    }
    fn to_feet(&self) -> Feet {
        Feet(self.value() / 0.3048)
    }
}

impl Feet {
    fn value(&self) -> f64 {
        self.0
    }
    fn to_meters(&self) -> Meters {
        Meters(self.value() * 0.3048)
    }
}

// R03: イテレータ + Result
// 【問題】文字列スライスを受け取り、
// 各文字列を i32 にパースして、偶数のみを集めた Vec を返してください。
// パース失敗は Err として返す。

fn parse_evens(strings: &[&str]) -> Result<Vec<i32>, String> {
    strings
        .iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map(|v| v.iter().filter(|n| **n % 2 == 0).copied().collect())
        .map_err(|e| e.to_string())
}

// ============================================================
// 【フェーズ8】async/await・Tokio
// ============================================================

// ============================================================
// Q01: async fn の基本
// ============================================================
// async fn は Future を返す関数。
// .await で Future の完了を待つ。
//
// async fn hello() -> String {
//     "hello".to_string()
// }
// // 呼び出し側:
// let s = hello().await;
//
// 【問題】以下の async 関数を実装してください。

// (1) 受け取った文字列をそのまま返す async 関数
async fn async_identity(s: String) -> String {
    s
}

// (2) 2つの i32 を受け取り、非同期に足し算する
async fn async_add(a: i32, b: i32) -> i32 {
    a + b
}

// (3) sleep して "done" を返す (100ms 待つ)
async fn delayed_done() -> &'static str {
    sleep(Duration::from_millis(100)).await;
    "done"
}

// ============================================================
// Q02: async fn の連鎖 (.await の使い方)
// ============================================================
// async fn の中で別の async fn を .await で呼び出せる。
// これが Scala の for-comprehension に相当する。
//
// Scala:
//   for {
//     a <- fetchA()
//     b <- fetchB(a)
//   } yield a + b
//
// Rust:
//   let a = fetch_a().await?;
//   let b = fetch_b(a).await?;
//   Ok(a + b)  // ← ? は Result の場合
//
// 【問題】以下の関数を async で実装してください。

// (1) ユーザーIDから名前を返す (疑似DB)
async fn fetch_name(id: u32) -> Option<String> {
    // 疑似的な非同期処理 (実際はDBやHTTPアクセスをイメージ)
    sleep(Duration::from_millis(1)).await;
    match id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        3 => Some("Carol".to_string()),
        _ => None,
    }
}

// (2) 名前からあいさつ文を作る
async fn fetch_greeting(id: u32) -> Option<String> {
    // fetch_name を await して "Hello, {name}!" を返す
    // id が見つからない場合は None
    let name = fetch_name(id).await?;
    Some(format!("Hello, {}!", name))
}

// ============================================================
// Q03: tokio::join! (並列実行)
// ============================================================
// join! は複数の Future を同時に実行し、全て完了を待つ。
// Scala の IO.both / parMapN に相当。
//
// let (a, b) = tokio::join!(future_a, future_b);
// // a と b は並列に実行される
//
// ★ .await を順番に書くと直列になる (遅い)
// ★ join! を使うと並列になる (速い)
//
// 【問題】3つの async 関数を join! で並列実行し、結果を合計してください。

async fn fetch_score_a() -> i32 {
    sleep(Duration::from_millis(10)).await;
    85
}

async fn fetch_score_b() -> i32 {
    sleep(Duration::from_millis(10)).await;
    92
}

async fn fetch_score_c() -> i32 {
    sleep(Duration::from_millis(10)).await;
    78
}

// 3つのスコアを並列取得して合計を返す
async fn total_score() -> i32 {
    let (a, b, c) = tokio::join!(fetch_score_a(), fetch_score_b(), fetch_score_c());
    a + b + c
}

// ============================================================
// Q04: tokio::spawn (タスクの生成)
// ============================================================
// spawn は新しいタスク(軽量スレッド)を起動する。
// Scala の IO.start / Future { ... } に相当。
//
// let handle = tokio::spawn(async {
//     // 別タスクで実行
//     expensive_work().await
// });
// let result = handle.await.unwrap(); // JoinHandle で結果を受け取る
//
// 【問題】n個のタスクを並列に spawn して、
// 各タスクが自分の番号を返す。全結果を集めて昇順ソートして返してください。

async fn spawn_tasks(n: u32) -> Vec<u32> {
    // ヒント:
    //   let handles: Vec<_> = (0..n).map(|i| {
    //       tokio::spawn(async move { i })
    //   }).collect();
    //   let mut results = Vec::new();
    //   for h in handles { results.push(h.await.unwrap()); }
    //   results.sort();
    let handles: Vec<_> = (0..n).map(|i| tokio::spawn(async move { i })).collect();
    let mut results = Vec::new();
    for h in handles {
        let res = h.await.unwrap();
        results.push(res);
    }
    results.sort();
    results
}

// ============================================================
// Q05: tokio::select! (最初に完了した方を使う)
// ============================================================
// select! は複数の Future のうち、最初に完了したものを使う。
// Scala の IO.race に相当。
//
// tokio::select! {
//     val = future_a => println!("a won: {}", val),
//     val = future_b => println!("b won: {}", val),
// }
//
// 【問題】2つの async 処理を race させ、
// 先に完了した方の結果を返してください。

async fn slow_task() -> &'static str {
    sleep(Duration::from_millis(100)).await;
    "slow"
}

async fn fast_task() -> &'static str {
    sleep(Duration::from_millis(10)).await;
    "fast"
}

async fn race_tasks() -> &'static str {
    tokio::select! {
        result = slow_task() => result,
        result = fast_task() => result,
    }
}

// ============================================================
// Q06: 非同期エラーハンドリング
// ============================================================
// async fn でも Result が使える。? 演算子も動く。
// Scala の IO[Either[E, A]] に相当。
//
// async fn fetch() -> Result<String, String> {
//     let data = risky_op().await?;
//     Ok(data)
// }
//
// 【問題】以下の疑似 HTTP クライアントを実装してください。

async fn mock_http_get(url: &str) -> Result<String, String> {
    sleep(Duration::from_millis(1)).await;
    match url {
        "http://ok.com" => Ok(r#"{"status":"ok"}"#.to_string()),
        "http://error.com" => Err("500 Internal Server Error".to_string()),
        _ => Err("404 Not Found".to_string()),
    }
}

// (1) URL を受け取り、レスポンスに "ok" が含まれれば true を返す
//     HTTP エラーは Err として伝播させる
async fn check_health(url: &str) -> Result<bool, String> {
    let response = mock_http_get(url).await?;
    Ok(response.contains("ok"))
}

// (2) 複数 URL を順番に試し、最初に成功したレスポンスを返す
//     全て失敗したら Err("all failed")
async fn fetch_first_ok(urls: &[&str]) -> Result<String, String> {
    // ヒント: for url in urls { if let Ok(r) = mock_http_get(url).await { return Ok(r); } }
    for url in urls {
        if let Ok(r) = mock_http_get(url).await {
            return Ok(r);
        }
    }
    Err("all failed".into())
}

// ============================================================
// Q07: 非同期チャンネル (mpsc)
// ============================================================
// Tokio の mpsc (multi-producer single-consumer) チャンネル。
// Scala の Queue / Akka の Actor メッセージングに相当。
//
// let (tx, mut rx) = tokio::sync::mpsc::channel(32);  // バッファサイズ32
//
// // 送信側
// tx.send(value).await.unwrap();
//
// // 受信側
// while let Some(msg) = rx.recv().await { ... }
//
// 【問題】producer/consumer パターンを実装してください。
// producer: 1..=n の値をチャンネルに送信
// consumer: 受け取った値を全て合計して返す

async fn sum_via_channel(n: u32) -> u32 {
    // ヒント:
    //   let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    //   tokio::spawn(async move {
    //       for i in 1..=n { tx.send(i).await.unwrap(); }
    //   });
    //   let mut sum = 0;
    //   while let Some(v) = rx.recv().await { sum += v; }
    //   sum
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        for i in 1..=n {
            tx.send(i).await.unwrap();
        }
    });
    let mut sum = 0;
    while let Some(v) = rx.recv().await {
        sum += v;
    }
    sum
}

// ============================================================
// Q08: 非同期 Mutex (共有状態)
// ============================================================
// 複数タスクで共有する状態は Arc<Mutex<T>> を使う。
//
//   std::sync::Mutex  ... 同期用 (async 内で使うと deadlock の危険)
//   tokio::sync::Mutex ... 非同期用 (async 内で安全に使える)
//
// Scala の Ref (cats-effect) や AtomicInteger に相当。
//
// let counter = Arc::new(tokio::sync::Mutex::new(0));
// let c = counter.clone();
// tokio::spawn(async move {
//     let mut lock = c.lock().await;
//     *lock += 1;
// });
//
// 【問題】n個のタスクを並列に走らせ、
// それぞれが共有カウンターを1増やす。最終値を返してください。

use std::sync::Arc;

async fn parallel_counter(n: u32) -> u32 {
    let counter = Arc::new(tokio::sync::Mutex::new(0u32));
    let handles: Vec<_> = (0..n)
        .map(|_| {
            let c = counter.clone();
            tokio::spawn(async move {
                let mut lock = c.lock().await;
                *lock += 1;
            })
        })
        .collect();
    for h in handles {
        h.await.unwrap();
    }
    let x = *counter.lock().await;
    x
}

// ============================================================
// Q09: 非同期イテレータ (Stream)
// ============================================================
// Tokio の Stream は非同期版の Iterator。
// Scala の fs2.Stream / Akka Streams に相当。
//
// ここでは Stream の代わりに
// チャンネル + collect パターンで同等のことを実現する。
//
// 【問題】フィボナッチ数列を非同期で生成し、
// 最初の n 項を Vec<u64> で返してください。
// 各項の生成に 1ms の遅延を入れること。

async fn async_fibonacci(n: usize) -> Vec<u64> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(64);
    tokio::spawn(async move {
        let (mut a, mut b) = (0u64, 1u64);
        for _ in 0..n {
            tx.send(a).await.unwrap();
            sleep(Duration::from_millis(1)).await;
            let next = a + b;
            a = b;
            b = next;
        }
    });
    let mut result = Vec::new();
    while let Some(v) = rx.recv().await {
        result.push(v);
    }
    result
}

// ============================================================
// Q10: 総合問題 - 非同期タスクスケジューラー
// ============================================================
// 【問題】シンプルな非同期タスクスケジューラーを実装してください。
//
// TaskScheduler:
//   - add_task(name, delay_ms, value): タスクを登録
//   - run_all(): 全タスクを並列実行し、完了した順に結果を返す
//
// 各タスクは delay_ms だけ待ってから value を返す。
// 結果は完了した順の Vec<(String, i32)> で返す。
// (delay が短いタスクが先に来る)

use tokio::sync::mpsc;

struct TaskScheduler {
    tasks: Vec<(String, u64, i32)>, // (name, delay_ms, value)
}

impl TaskScheduler {
    fn new() -> TaskScheduler {
        TaskScheduler { tasks: Vec::new() }
    }

    fn add_task(&mut self, name: &str, delay_ms: u64, value: i32) {
        self.tasks.push((name.to_string(), delay_ms, value));
    }

    async fn run_all(self) -> Vec<(String, i32)> {
        let (tx, mut rx) = mpsc::channel(self.tasks.len() + 1);
        for (name, delay, value) in self.tasks {
            let tx = tx.clone();
            tokio::spawn(async move {
                sleep(Duration::from_millis(delay)).await;
                tx.send((name, value)).await.unwrap();
            });
        }
        drop(tx);
        let mut results = Vec::new();
        while let Some(r) = rx.recv().await {
            results.push(r);
        }
        results
    }
}

// ============================================================
// テスト (変更不要)
// ============================================================

// --- 復習 ---
#[test]
fn test_r01_converter() {
    assert!((converter::celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-9);
    assert!((converter::fahrenheit_to_celsius(32.0) - 0.0).abs() < 1e-9);
    assert!((converter::celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-9);
}

#[test]
fn test_r02_newtype() {
    let m = Meters::new(1.0);
    assert!((m.to_feet().value() - (1.0 / 0.3048)).abs() < 1e-9);
    let f = Feet(1.0);
    assert!((f.to_meters().value() - 0.3048).abs() < 1e-9);
}

#[test]
fn test_r03_parse_evens() {
    assert_eq!(parse_evens(&["2", "4", "6"]), Ok(vec![2, 4, 6]));
    assert_eq!(parse_evens(&["1", "2", "3"]), Ok(vec![2]));
    assert!(parse_evens(&["1", "x"]).is_err());
}

// --- Q01 ---
#[tokio::test]
async fn test_q01_identity() {
    assert_eq!(async_identity("hello".to_string()).await, "hello");
}

#[tokio::test]
async fn test_q01_add() {
    assert_eq!(async_add(3, 4).await, 7);
}

#[tokio::test]
async fn test_q01_delayed() {
    let result = delayed_done().await;
    assert_eq!(result, "done");
}

// --- Q02 ---
#[tokio::test]
async fn test_q02_fetch_greeting_found() {
    assert_eq!(fetch_greeting(1).await, Some("Hello, Alice!".to_string()));
    assert_eq!(fetch_greeting(2).await, Some("Hello, Bob!".to_string()));
}

#[tokio::test]
async fn test_q02_fetch_greeting_not_found() {
    assert_eq!(fetch_greeting(99).await, None);
}

// --- Q03 ---
#[tokio::test]
async fn test_q03_total_score() {
    assert_eq!(total_score().await, 255);
}

// --- Q04 ---
#[tokio::test]
async fn test_q04_spawn_tasks() {
    let result = spawn_tasks(5).await;
    assert_eq!(result, vec![0, 1, 2, 3, 4]);
}

#[tokio::test]
async fn test_q04_spawn_empty() {
    let result = spawn_tasks(0).await;
    assert_eq!(result, vec![]);
}

// --- Q05 ---
#[tokio::test]
async fn test_q05_race() {
    // fast_task (10ms) が slow_task (100ms) より先に完了するはず
    assert_eq!(race_tasks().await, "fast");
}

// --- Q06 ---
#[tokio::test]
async fn test_q06_health_ok() {
    assert_eq!(check_health("http://ok.com").await, Ok(true));
}

#[tokio::test]
async fn test_q06_health_error() {
    assert!(check_health("http://error.com").await.is_err());
}

#[tokio::test]
async fn test_q06_fetch_first_ok() {
    let result = fetch_first_ok(&["http://error.com", "http://ok.com"]).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_q06_fetch_all_fail() {
    let result = fetch_first_ok(&["http://error.com", "http://none.com"]).await;
    assert!(result.is_err());
}

// --- Q07 ---
#[tokio::test]
async fn test_q07_channel_sum() {
    assert_eq!(sum_via_channel(10).await, 55);
    assert_eq!(sum_via_channel(0).await, 0);
}

// --- Q08 ---
#[tokio::test]
async fn test_q08_counter() {
    assert_eq!(parallel_counter(10).await, 10);
    assert_eq!(parallel_counter(100).await, 100);
}

// --- Q09 ---
#[tokio::test]
async fn test_q09_fib() {
    let result = async_fibonacci(8).await;
    assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13]);
}

// --- Q10 ---
#[tokio::test]
async fn test_q10_scheduler_order() {
    let mut sched = TaskScheduler::new();
    sched.add_task("slow", 50, 1);
    sched.add_task("fast", 10, 2);
    sched.add_task("medium", 30, 3);

    let results = sched.run_all().await;

    // 完了順: fast(10ms) -> medium(30ms) -> slow(50ms)
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].0, "fast");
    assert_eq!(results[1].0, "medium");
    assert_eq!(results[2].0, "slow");
}

#[tokio::test]
async fn test_q10_scheduler_values() {
    let mut sched = TaskScheduler::new();
    sched.add_task("a", 10, 100);
    sched.add_task("b", 20, 200);
    let results = sched.run_all().await;
    assert_eq!(results[0], ("a".to_string(), 100));
    assert_eq!(results[1], ("b".to_string(), 200));
}
