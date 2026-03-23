// ============================================================
// Rust学習 フェーズ7: モジュール・クレート・プロジェクト構成
// ============================================================
// 実行方法:
//   cargo test --test phase7_modules          # このファイル全体
//   cargo test --test phase7_modules test_r01 # 復習だけ
//   cargo test --test phase7_modules test_q01 # 新問Q1だけ
//
// Scalaとの対応:
//   mod          ≒ package object / object
//   pub          ≒ public (デフォルトは private、Scalaと逆！)
//   use          ≒ import
//   crate        ≒ sbt プロジェクト / jar
//   Cargo.toml   ≒ build.sbt
//   extern crate ≒ libraryDependencies (現代では不要)
//
// ★ フェーズ7の特性について
//   モジュールシステムはファイル分割・可視性制御が中心のため、
//   このファイル内では mod ブロックを使ってモジュールを模擬します。
//   実際のプロジェクトでは src/lib.rs + src/サブディレクトリ で構成します。
// ============================================================

// ============================================================
// 【フェーズ6 復習】
// ============================================================

// R01: イテレータメソッドチェーン
// 【問題】数値スライスを受け取り、
// 奇数だけ取り出して3倍にし、合計を返してください。
// for ループ禁止・メソッドチェーンのみ。

fn triple_odds_sum(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|n| **n % 2 != 0)
        .map(|n| *n * 3)
        .sum()
}

// R02: flat_map + filter
// 【問題】文章のスライスを受け取り、
// 全単語の中から長さ4以上のものだけを Vec<String> で返してください。
// 例: ["hello world", "hi rust"] => ["hello", "world", "rust"]

fn long_words(sentences: &[&str]) -> Vec<String> {
    sentences
        .iter()
        .flat_map(|sentence| sentence.split_whitespace())
        .filter(|w| w.len() >= 4)
        .map(|w| w.to_string())
        .collect()
}

// R03: カスタム Iterator
// 【問題】CountDown イテレータを実装してください。
// new(n) で作成し、n, n-1, ..., 1, 0 の順に値を返す。

struct CountDown {
    current: i32,
}

impl CountDown {
    fn new(n: i32) -> CountDown {
        Self { current: n }
    }
}

impl Iterator for CountDown {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let ret = self.current;
        if ret < 0 {
            None
        } else {
            self.current = self.current - 1;
            Some(ret)
        }
    }
}

// ============================================================
// 【フェーズ7】モジュール・可視性・クレート
// ============================================================

// ============================================================
// Q01: mod ブロックと可視性 (pub)
// ============================================================
// Rust のデフォルト可視性は private (Scala と逆！)。
// pub をつけると外から見える。
//
// mod の種類:
//   pub fn        ... 公開関数
//   pub struct    ... 公開構造体 (フィールドは別途 pub が必要)
//   pub(crate)    ... クレート内だけ公開
//   pub(super)    ... 親モジュールだけ公開
//
// 【問題】以下の geometry モジュールを完成させてください。
// - Point は pub (フィールドも pub)
// - distance 関数は pub
// - helper 関数は pub にしない (モジュール内部用)

mod geometry {
    // 【問題1】pub な Point 構造体を定義 (x: f64, y: f64)
    // todo: Point 構造体を定義してください
    pub struct Point {
        x: f64,
        y: f64,
    }

    // 【問題2】2点間の距離を返す pub 関数
    // ヒント: ((x2-x1)² + (y2-y1)²).sqrt()
    pub fn distance(p1: &Point, p2: &Point) -> f64 {
        (sq_diff(p1.x, p2.x) + sq_diff(p1.y, p2.y)).sqrt()
    }

    // 【問題3】差の二乗を返す (pub にしない)
    fn sq_diff(a: f64, b: f64) -> f64 {
        (a - b).powi(2)
    }

    // 【問題4】Point の new 関連関数を impl ブロックで定義
    impl Point {
        pub fn new(x: f64, y: f64) -> Point {
            Self { x, y }
        }
    }
}

// ============================================================
// Q02: use による名前空間のインポート
// ============================================================
// use は Scala の import とほぼ同じ。
// Rust では use はスコープ内どこでも書ける。
//
// よくある use のパターン:
//   use std::collections::HashMap;
//   use std::collections::{HashMap, HashSet};  // 複数まとめて
//   use std::collections::*;                    // glob (非推奨)
//   use crate::geometry::Point;                 // 同クレート内
//   use super::geometry::Point;                 // 親モジュールから
//
// 【問題】以下の calculator モジュールを実装してください。
// ops サブモジュール内に四則演算を定義し、
// calculator モジュールから use で使う。

mod calculator {
    mod ops {
        pub fn add(a: f64, b: f64) -> f64 {
            a + b
        }
        pub fn sub(a: f64, b: f64) -> f64 {
            a - b
        }
        pub fn mul(a: f64, b: f64) -> f64 {
            a * b
        }
        pub fn div(a: f64, b: f64) -> Option<f64> {
            // ゼロ除算は None
            if b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
    }

    // 【問題】ops の関数を use してから使う eval 関数を実装してください。
    // op: '+', '-', '*', '/' のいずれか
    // 対応しない op や ゼロ除算は None を返す
    pub fn eval(a: f64, op: char, b: f64) -> Option<f64> {
        use ops::{add, div, mul, sub};
        match op {
            '+' => Some(add(a, b)),
            '-' => Some(sub(a, b)),
            '*' => Some(mul(a, b)),
            '/' => div(a, b),
            _ => None,
        }
    }
}

// ============================================================
// Q03: 構造体のフィールド可視性
// ============================================================
// struct を pub にしても、フィールドはデフォルト private。
// フィールドを pub にするか、pub なアクセサを提供するのが Rust スタイル。
//
// 【問題】BankAccount 構造体を実装してください。
// - balance フィールドは非公開 (外から直接変更できない)
// - deposit / withdraw / balance メソッドを pub で提供
// - withdraw は残高不足なら Err を返す

mod bank {
    #[derive(Debug)]
    pub struct BankAccount {
        owner: String,
        balance: f64, // pub にしない
    }

    impl BankAccount {
        pub fn new(owner: &str, initial: f64) -> BankAccount {
            Self {
                owner: owner.to_string(),
                balance: initial,
            }
        }

        pub fn deposit(&mut self, amount: f64) {
            self.balance = self.balance + amount;
        }

        pub fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
            // 成功: Ok(引き出し後の残高)
            // 失敗: Err("Insufficient funds")
            let diff = self.balance - amount;
            if diff >= 0.0 {
                self.balance = diff;
                Ok(diff)
            } else {
                Err("Insufficient funds".into())
            }
        }

        pub fn balance(&self) -> f64 {
            self.balance
        }

        pub fn owner(&self) -> &str {
            &self.owner
        }
    }
}

// ============================================================
// Q04: 型エイリアス (type)
// ============================================================
// type キーワードで型に別名をつけられる。
// Scala の type エイリアスと同じ。
//
// 用途:
//   - 長い型を短く書く
//   - ドメイン固有の意味を持たせる
//   - Result 型のデフォルトエラーを固定する
//
// 例:
//   type Meters = f64;
//   type AppResult<T> = Result<T, AppError>;

// 【問題1】以下の型エイリアスを定義してください。
// Kilometers, Meters, Seconds は全て f64 の別名

type Kilometers = f64;
// todo: Meters と Seconds も定義してください
type Meters = f64;
type Seconds = f64;

// 【問題2】型エイリアスを使う関数を実装してください。
// speed(distance_km, time_sec) => m/s での速度を返す

fn speed(distance: Kilometers, time: Seconds) -> f64 {
    distance * 1000.0 / time
}

// 【問題3】Result の型エイリアス
// ParseResult<T> = Result<T, String> の型エイリアスを定義し、
// 文字列を f64 にパースする関数を実装してください。

type ParseResult<T> = Result<T, String>;

fn parse_f64(s: &str) -> ParseResult<f64> {
    s.parse::<f64>().map_err(|e| e.to_string())
}

// ============================================================
// Q05: 定数と静的変数
// ============================================================
// const  ... コンパイル時定数。型注釈必須。インライン展開される。
//            Scala の final val と同じ感覚。
// static ... プログラム全体で1つの場所に存在するグローバル変数。
//            static mut は unsafe が必要 (基本的に避ける)。
//
// 【問題】以下の定数を使って単位変換関数を実装してください。

const CM_PER_INCH: f64 = 2.54;
const METERS_PER_FOOT: f64 = 0.3048;
const GRAMS_PER_POUND: f64 = 453.592;

// (1) インチをセンチメートルに変換
fn inches_to_cm(inches: f64) -> f64 {
    inches * CM_PER_INCH
}

// (2) フィートをメートルに変換
fn feet_to_meters(feet: f64) -> f64 {
    feet * METERS_PER_FOOT
}

// (3) ポンドをグラムに変換
fn pounds_to_grams(pounds: f64) -> f64 {
    pounds * GRAMS_PER_POUND
}

// ============================================================
// Q06: newtype パターン
// ============================================================
// タプル構造体 1フィールド版 = "newtype"。
// 既存の型に型安全な別名をつける Rust のイディオム。
// Scala の value class / opaque type に相当。
//
// メリット: コンパイル時に単位の混在を防げる
//
// 例:
//   struct Meters(f64);
//   struct Kilograms(f64);
//   // Meters と Kilograms は別の型なので混在するとコンパイルエラー!

struct Celsius(f64);
struct Fahrenheit(f64);
struct Kelvin(f64);

impl Celsius {
    fn new(v: f64) -> Celsius {
        Celsius(v)
    }
    fn value(&self) -> f64 {
        self.0
    }

    // 【問題1】Celsius -> Fahrenheit 変換
    fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit(self.value() * 9.0 / 5.0 + 32.0)
    }

    // 【問題2】Celsius -> Kelvin 変換 (K = C + 273.15)
    fn to_kelvin(&self) -> Kelvin {
        Kelvin(self.value() + 273.15)
    }
}

impl Fahrenheit {
    fn value(&self) -> f64 {
        self.0
    }
}

impl Kelvin {
    fn value(&self) -> f64 {
        self.0
    }
}

// ============================================================
// Q07: prelude パターンと pub use (再エクスポート)
// ============================================================
// ライブラリクレートでよく使う pub use による再エクスポート。
// Scala の package object での再エクスポートに相当。
//
// ユーザーが use my_lib::prelude::* と書くだけで
// よく使う型が全部使えるようになる設計パターン。
//
// 【問題】以下の shapes モジュールを実装し、
// prelude サブモジュールで主要な型を再エクスポートしてください。

mod shapes {
    pub struct Circle {
        pub radius: f64,
    }

    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    pub trait Area {
        fn area(&self) -> f64;
    }

    // 【問題1】Circle に Area を実装
    impl Area for Circle {
        fn area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }
    }

    // 【問題2】Rectangle に Area を実装
    impl Area for Rectangle {
        fn area(&self) -> f64 {
            self.height * self.width
        }
    }

    // 【問題3】prelude サブモジュールで Circle, Rectangle, Area を再エクスポート
    pub mod prelude {
        pub use super::Area;
        pub use super::Circle;
        pub use super::Rectangle;
    }
}

// ============================================================
// Q08: Cargo.toml の依存関係 (知識問題)
// ============================================================
// 実際にコードを書く問題ではなく、Cargo.toml の書き方を理解する問題。
//
// 【問題】以下の要件を満たす Cargo.toml の [dependencies] セクションを
// コメントとして書いてください。その後、serde の機能を使うコードを実装してください。
//
// 要件:
//   - serde バージョン 1.0 (features: derive)
//   - serde_json バージョン 1.0
//
// Cargo.toml に書く内容 (参考):
//   [dependencies]
//   serde = { version = "1.0", features = ["derive"] }
//   serde_json = "1.0"
//
// ★ このファイルでは serde を使わず、同等の機能を手動実装します。
//   実際のプロジェクトでは serde を使うと JSON シリアライズが簡単になる。

// JSON 風の文字列に変換するトレイト (serde の Serialize に相当)
trait ToJson {
    fn to_json(&self) -> String;
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

impl Person {
    fn new(name: &str, age: u32, email: &str) -> Person {
        Person {
            name: name.to_string(),
            age,
            email: email.to_string(),
        }
    }
}

// 【問題】Person に ToJson を実装してください。
// 出力形式: {"name":"Alice","age":30,"email":"alice@example.com"}

impl ToJson for Person {
    fn to_json(&self) -> String {
        format!(
            r#"{{"name":"{}","age":{},"email":"{}"}}"#,
            self.name, self.age, self.email
        )
    }
}

// ============================================================
// Q09: ワークスペースの概念 + 条件コンパイル
// ============================================================
// #[cfg(...)] アトリビュートで条件付きコンパイルができる。
// Scala の scalacOptions / conditional compilation に相当。
//
// よく使うもの:
//   #[cfg(test)]           テスト時のみコンパイル
//   #[cfg(debug_assertions)] デバッグビルド時のみ
//   #[cfg(target_os = "linux")] Linux のみ
//   #[cfg(feature = "serde")] feature フラグ
//
// 【問題】以下を実装してください。

// (1) デバッグ用のログ関数 (debug_assertions 時のみ出力)
// 本番ビルドでは何もしない (コンパイルから除外)
fn debug_log(msg: &str) {
    #[cfg(debug_assertions)]
    println!("[DEBUG] {}", msg);
}

// (2) プラットフォームに応じた改行コードを返す関数
fn line_ending() -> &'static str {
    if cfg!(target_os = "windows") {
        "\r\n"
    } else {
        "\n"
    }
}

// (3) テスト用ヘルパー関数 (テスト時のみ存在する)
// 【問題】以下の関数を、テスト時のみコンパイルされるよう実装してください。
#[cfg(test)]
fn make_test_person() -> Person {
    // Alice, 30, alice@example.com
    Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    }
}

// ============================================================
// Q10: 総合問題 - 小さなライブラリ設計
// ============================================================
// 【問題】簡易的なタスク管理ライブラリを実装してください。
// モジュール構成と可視性を意識して設計する。

mod todo_lib {
    #[derive(Debug, PartialEq, Clone)]
    pub enum Priority {
        Low,
        Medium,
        High,
    }

    #[derive(Debug, Clone)]
    pub struct Task {
        id: u32,
        pub title: String,
        pub done: bool,
        pub priority: Priority,
    }

    impl Task {
        pub fn new(id: u32, title: &str, priority: Priority) -> Task {
            Self {
                id,
                title: title.to_string(),
                done: false,
                priority,
            }
        }

        pub fn id(&self) -> u32 {
            self.id
        }

        pub fn complete(&mut self) {
            self.done = true;
        }
    }

    pub struct TaskManager {
        tasks: Vec<Task>,
        next_id: u32,
    }

    impl TaskManager {
        pub fn new() -> TaskManager {
            Self {
                tasks: Vec::new(),
                next_id: 0,
            }
        }

        // タスクを追加し、割り当てられた ID を返す
        pub fn add(&mut self, title: &str, priority: Priority) -> u32 {
            let id = self.next_id;
            self.tasks.push(Task::new(id, title, priority));
            self.next_id = id + 1;
            id
        }

        // ID でタスクを完了にする。見つからなければ Err
        pub fn complete(&mut self, id: u32) -> Result<(), String> {
            self.tasks
                .iter_mut()
                .find(|task| task.id() == id)
                .map(|task| task.complete())
                .ok_or("Task not found".into())
        }

        // 未完了タスクの一覧を返す (優先度 High -> Medium -> Low の順)
        pub fn pending(&self) -> Vec<&Task> {
            let mut result: Vec<&Task> = self.tasks.iter().filter(|t| !t.done).collect();
            result.sort_by_key(|t| match t.priority {
                Priority::High => 0,
                Priority::Medium => 1,
                Priority::Low => 2,
            });
            result
        }

        // 完了済みタスクの数を返す
        pub fn done_count(&self) -> usize {
            self.tasks.iter().filter(|t| t.done).count()
        }
    }
}

// ============================================================
// テスト (変更不要)
// ============================================================

// --- 復習 ---
#[test]
fn test_r01_triple_odds() {
    assert_eq!(triple_odds_sum(&[1, 2, 3, 4, 5]), 27); // 1*3 + 3*3 + 5*3
    assert_eq!(triple_odds_sum(&[2, 4, 6]), 0);
}

#[test]
fn test_r02_long_words() {
    let result = long_words(&["hello world", "hi rust"]);
    assert_eq!(result, vec!["hello", "world", "rust"]);
}

#[test]
fn test_r03_countdown() {
    let v: Vec<i32> = CountDown::new(3).collect();
    assert_eq!(v, vec![3, 2, 1, 0]);
}
#[test]
fn test_r03_countdown_zero() {
    let v: Vec<i32> = CountDown::new(0).collect();
    assert_eq!(v, vec![0]);
}

// --- Q01 ---
#[test]
fn test_q01_distance() {
    let p1 = geometry::Point::new(0.0, 0.0);
    let p2 = geometry::Point::new(3.0, 4.0);
    assert!((geometry::distance(&p1, &p2) - 5.0).abs() < 1e-9);
}
#[test]
fn test_q01_same_point() {
    let p = geometry::Point::new(1.0, 2.0);
    let q = geometry::Point::new(1.0, 2.0);
    assert!((geometry::distance(&p, &q) - 0.0).abs() < 1e-9);
}

// --- Q02 ---
#[test]
fn test_q02_eval_add() {
    assert_eq!(calculator::eval(3.0, '+', 4.0), Some(7.0));
}
#[test]
fn test_q02_eval_div_zero() {
    assert_eq!(calculator::eval(1.0, '/', 0.0), None);
}
#[test]
fn test_q02_eval_unknown_op() {
    assert_eq!(calculator::eval(1.0, '%', 2.0), None);
}
#[test]
fn test_q02_eval_mul() {
    assert_eq!(calculator::eval(3.0, '*', 4.0), Some(12.0));
}

// --- Q03 ---
#[test]
fn test_q03_bank_deposit() {
    let mut acc = bank::BankAccount::new("Alice", 100.0);
    acc.deposit(50.0);
    assert!((acc.balance() - 150.0).abs() < 1e-9);
}
#[test]
fn test_q03_bank_withdraw_ok() {
    let mut acc = bank::BankAccount::new("Bob", 200.0);
    let result = acc.withdraw(80.0);
    assert!(result.is_ok());
    assert!((acc.balance() - 120.0).abs() < 1e-9);
}
#[test]
fn test_q03_bank_withdraw_fail() {
    let mut acc = bank::BankAccount::new("Carol", 50.0);
    assert!(acc.withdraw(100.0).is_err());
    assert!((acc.balance() - 50.0).abs() < 1e-9); // 変わっていない
}

// --- Q04 ---
#[test]
fn test_q04_speed() {
    // 1km を 1000秒で走ると 1.0 m/s
    assert!((speed(1.0, 1000.0) - 1.0).abs() < 1e-9);
}
#[test]
fn test_q04_parse_f64() {
    assert!((parse_f64("3.14").unwrap() - 3.14).abs() < 1e-9);
    assert!(parse_f64("abc").is_err());
}

// --- Q05 ---
#[test]
fn test_q05_conversions() {
    assert!((inches_to_cm(1.0) - 2.54).abs() < 1e-9);
    assert!((feet_to_meters(1.0) - 0.3048).abs() < 1e-9);
    assert!((pounds_to_grams(1.0) - 453.592).abs() < 0.001);
}

// --- Q06 ---
#[test]
fn test_q06_celsius_to_f() {
    let c = Celsius::new(0.0);
    assert!((c.to_fahrenheit().value() - 32.0).abs() < 1e-9);
    let c2 = Celsius::new(100.0);
    assert!((c2.to_fahrenheit().value() - 212.0).abs() < 1e-9);
}
#[test]
fn test_q06_celsius_to_k() {
    let c = Celsius::new(0.0);
    assert!((c.to_kelvin().value() - 273.15).abs() < 1e-9);
}

// --- Q07 ---
#[test]
fn test_q07_shapes_prelude() {
    use shapes::prelude::*;
    let c = Circle { radius: 1.0 };
    assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
    let r = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert!((r.area() - 12.0).abs() < 1e-9);
}

// --- Q08 ---
#[test]
fn test_q08_to_json() {
    let p = Person::new("Alice", 30, "alice@example.com");
    assert_eq!(
        p.to_json(),
        r#"{"name":"Alice","age":30,"email":"alice@example.com"}"#
    );
}

// --- Q09 ---
#[test]
fn test_q09_line_ending() {
    let ending = line_ending();
    assert!(ending == "\n" || ending == "\r\n");
}
#[test]
fn test_q09_make_test_person() {
    let p = make_test_person();
    assert_eq!(p.name, "Alice");
    assert_eq!(p.age, 30);
}

// --- Q10 ---
#[test]
fn test_q10_task_manager() {
    use todo_lib::{Priority, TaskManager};
    let mut mgr = TaskManager::new();
    let _id1 = mgr.add("Buy milk", Priority::Low);
    let id2 = mgr.add("Fix bug", Priority::High);
    let _id3 = mgr.add("Write docs", Priority::Medium);

    assert_eq!(mgr.pending().len(), 3);
    assert_eq!(mgr.done_count(), 0);

    // 優先度順: High -> Medium -> Low
    let pending = mgr.pending();
    assert_eq!(pending[0].priority, Priority::High);
    assert_eq!(pending[1].priority, Priority::Medium);
    assert_eq!(pending[2].priority, Priority::Low);

    // タスク完了
    assert!(mgr.complete(id2).is_ok());
    assert_eq!(mgr.pending().len(), 2);
    assert_eq!(mgr.done_count(), 1);

    // 存在しない ID
    assert!(mgr.complete(999).is_err());
}
