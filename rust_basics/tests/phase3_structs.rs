// ============================================================
// Rust学習 フェーズ3: 構造体・列挙型・パターンマッチ
// ============================================================
// 実行方法:
//   cargo test --test phase3_structs          # このファイル全体
//   cargo test --test phase3_structs test_r01 # 復習だけ
//   cargo test --test phase3_structs test_q01 # 新問Q1だけ
//
// Scalaとの対応:
//   struct          ≒ case class
//   enum            ≒ sealed trait + case class
//   impl            ≒ class のメソッド定義
//   pattern match   ≒ match/case (ほぼ同じ！)
// ============================================================

// ============================================================
// 【フェーズ2 復習】
// ============================================================

// R01: 不変借用
// 【問題】i32スライスの平均値を返してください。空なら None。

fn average(numbers: &[i32]) -> Option<f64> {
    if numbers.is_empty() {
        None
    } else {
        let sum = numbers.iter().copied().sum::<i32>() as f64;
        Some(sum / numbers.len() as f64)
    }
}

// R02: 可変借用
// 【問題】Vec<String> への可変参照を受け取り、
// 全要素を大文字に変換してください (戻り値なし)。
// ヒント: s.to_uppercase() は新しいStringを返す

fn uppercase_all(words: &mut Vec<String>) {
    for word in words {
        *word = word.to_uppercase();
    }
}

// R03: 文字列スライス
// 【問題】文字列を受け取り、最後の単語を返してください。
// スペースがなければ文字列全体を返す。
// 例: last_word("hello world foo") => "foo"

fn last_word(s: &str) -> &str {
    // ヒント: s.rfind(' ') で後ろからスペースを探せる
    s.rfind(' ').map_or(s, |i| &s[i+1..])
}

// ============================================================
// 【フェーズ3】構造体・列挙型・パターンマッチ
// ============================================================

// ============================================================
// Q01: 構造体 (struct) の定義とインスタンス生成
// ============================================================
// Rustの struct は Scala の case class に近い。
//
// 定義例:
//   struct Point { x: f64, y: f64 }
//
// 生成例:
//   let p = Point { x: 1.0, y: 2.0 };
//   println!("{}", p.x);
//
// 【問題】以下の Rectangle 構造体に面積と周長を返すメソッドを実装してください。

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // 【問題1】面積 (width * height) を返す
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // 【問題2】周長 (2 * (width + height)) を返す
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    // 【問題3】正方形かどうかを返す
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// ============================================================
// Q02: コンストラクタパターン (new関連関数)
// ============================================================
// Rustに特別なコンストラクタ構文はないが、
// 慣例として impl ブロックに new() 関連関数を定義する。
// (Scalaの companion object の apply() に相当)
//
// 【問題】Circle 構造体と impl を完成させてください。

struct Circle {
    radius: f64,
}

impl Circle {
    // 【問題1】radius を受け取り Circle を生成する関連関数
    // (selfを取らない = Scalaのstaticメソッド相当)
    fn new(radius: f64) -> Circle {
        Self { radius }
    }

    // 【問題2】面積を返す (π * r²)
    // ヒント: std::f64::consts::PI または 3.14159265358979_f64
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    // 【問題3】2つのCircleを受け取り、大きい方を返す
    fn larger<'a>(c1: &'a Circle, c2: &'a Circle) -> &'a Circle {
        if c1.radius > c2.radius {
            c1
        } else {
            c2
        }
    }
}

// ============================================================
// Q03: 構造体の更新構文
// ============================================================
// Scala の copy() に相当する機能が Rust にもある:
//   let p2 = Point { x: 5.0, ..p1 };  // x だけ変えて残りはp1から
//
// 【問題】Point3D 構造体を定義し、
// z座標だけ変えた新しいPoint3Dを返すメソッド with_z を実装してください。

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Point3D {
        Self { x, y, z }
    }

    // z だけ変えた新しい Point3D を返す
    fn with_z(&self, new_z: f64) -> Point3D {
        // ヒント: Point3D { z: new_z, ..*self } という更新構文が使える
        //         *self は self の参照外し
        Point3D { z: new_z, ..*self }
    }
}

// ============================================================
// Q04: 列挙型 (enum) 基本
// ============================================================
// Rustの enum は Scala の sealed trait + case object に相当。
// 各バリアントがデータを持てるのが強力。
//
// 例:
//   enum Direction { North, South, East, West }
//
// 【問題】以下の Shape enum に面積を返すメソッドを実装してください。

enum Shape {
    Circle(f64),         // 半径
    Rectangle(f64, f64), // 幅, 高さ
    Triangle(f64, f64),  // 底辺, 高さ
}

impl Shape {
    fn area(&self) -> f64 {
        // ヒント: match self { Shape::Circle(r) => ..., ... }
        match *self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(a, b) => a * b,
            Shape::Triangle(a, b) => a * b / 2.0,
        }
    }
}

// ============================================================
// Q05: enum にデータを持たせる (Scalaのcase classに相当)
// ============================================================
// Rustのenumバリアントは構造体のようにフィールドを持てる。
//
// 【問題】以下の Message enum を処理する関数を実装してください。

enum Message {
    Quit,                    // データなし
    Move { x: i32, y: i32 }, // 名前付きフィールド
    Write(String),           // 文字列データ
    ChangeColor(u8, u8, u8), // RGB値
}

impl Message {
    // メッセージを人間が読める文字列に変換する
    fn describe(&self) -> String {
        // 例:
        //   Quit          => "Quit"
        //   Move{x:1,y:2} => "Move to (1, 2)"
        //   Write("hi")   => "Write: hi"
        //   ChangeColor(255,0,0) => "Color: (255, 0, 0)"
        match self {
            Message::Quit => "Quit".to_string(),
            Message::Move { x, y } => format!("Move to ({}, {})", x, y),
            Message::Write(s) => format!("Write: {}", s),
            Message::ChangeColor(r, g, b) => format!("Color: ({}, {}, {})", r, g, b),
        }
    }
}

// ============================================================
// Q06: Option を使いこなす
// ============================================================
// Scalaの Option と同じ概念だが、Rustのイディオムを学ぶ。
//
// 便利なメソッド:
//   opt.unwrap_or(default)    // Noneなら default を使う
//   opt.map(|x| x + 1)       // Someの中身を変換
//   opt.and_then(|x| f(x))   // flatMap に相当
//   opt.is_some() / is_none()
//
// 【問題】Vec<Option<i32>> を受け取り、
// Some の値だけを取り出して合計を返してください。
// 例: sum_options(vec![Some(1), None, Some(3), None, Some(5)]) => 9

fn sum_options(values: Vec<Option<i32>>) -> i32 {
    // ヒント: for v in values { if let Some(n) = v { ... } }
    //         または .flatten() でOption内の値だけ取り出せる
    values.iter().flatten().sum()
}

// ============================================================
// Q07: if let / while let
// ============================================================
// 1つのパターンだけマッチしたい場合は if let が便利。
// (Scalaの pattern matching の部分適用に相当)
//
// 例:
//   if let Some(x) = some_option { println!("{}", x); }
//
// 【問題】スタック (Vec<i32>) から値をポップし続け、
// 0 が出たら止まる関数を実装してください。
// 0より前にポップした値を Vec<i32> で返す。
// 例: スタック [5, 3, 0, 8, 1] (末尾が先頭) => [1, 8] (0の前の値)
// ※ スタックに0がなければ全部返す

fn pop_until_zero(stack: &mut Vec<i32>) -> Vec<i32> {
    // ヒント: while let Some(n) = stack.pop() { ... }
    let mut result = Vec::new();
    while let Some(n) = stack.pop() {
        if n == 0 {
            break;
        } else {
            result.push(n);
        }
    }
    result
}

// ============================================================
// Q08: パターンマッチの応用
// ============================================================
// Rustのmatchはガード条件や複数パターンも書ける。
//
// 例:
//   match n {
//     x if x < 0 => "negative",
//     0           => "zero",
//     1 | 2 | 3   => "small",
//     _           => "large",
//   }
//
// 【問題】i32 を受け取り、以下のルールで文字列を返してください:
//   - 負の数         => "negative"
//   - 0              => "zero"
//   - 1, 2, 3        => "small"
//   - 4..=9          => "medium"  (範囲パターン)
//   - 10以上で偶数   => "large even"
//   - 10以上で奇数   => "large odd"

fn classify(n: i32) -> &'static str {
    match n {
        x if x < 0 => "negative",
        0 => "zero",
        1..=3 => "small",
        4..=9 => "medium",
        x if x % 2 == 0 => "large even",
        _ => "large odd",
    }
}

// ============================================================
// Q09: 構造体のパターンマッチ (分解)
// ============================================================
// Scalaの case class の unapply に相当。
// Rustでは match や let で構造体を分解できる。
//
// 例:
//   let Point { x, y } = p;  // 分解
//   match p { Point { x, y } => println!("{} {}", x, y) }
//
// 【問題】以下の Color enum と、2色を混ぜる関数を実装してください。
// RGB値は各チャンネルの平均値とする。

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    // 2色を混ぜる (各チャンネルの平均、切り捨て)
    fn mix(&self, other: &Color) -> Color {
        // ヒント: ((self.r as u16 + other.r as u16) / 2) as u8
        //         u8のオーバーフローを避けるためu16にキャスト
        Color {
            r: ((self.r as u16 + other.r as u16) / 2) as u8,
            g: ((self.g as u16 + other.g as u16) / 2) as u8,
            b: ((self.b as u16 + other.b as u16) / 2) as u8,
        }
    }

    fn to_tuple(&self) -> (u8, u8, u8) {
       (self.r, self.g, self.b)
    }
}

// ============================================================
// Q10: 総合問題 - 簡易電卓
// ============================================================
// 【問題】以下の Expr enum を使った式評価器を実装してください。

enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    // 式を評価して結果を返す。0除算は None。
    fn eval(&self) -> Option<f64> {
        // ヒント:
        //   ★ ? 演算子: None なら即 None を返す (Scalaの flatMap に相当)
        //   ★ Box<Expr> は再帰的な enum に必要 (サイズをコンパイル時に決定するため)
        match self {
            Expr::Num(n) => Some(*n),
            Expr::Add(a, b) => Some(a.eval()? + b.eval()?),
            Expr::Sub(a, b) => Some(a.eval()? - b.eval()?),
            Expr::Mul(a, b) => Some(a.eval()? * b.eval()?),
            Expr::Div(a, b) => {
                let divisor = b.eval()?;
                if divisor == 0.0 {
                    None
                } else {
                    Some(a.eval()? / divisor)
                }
            }
        }
    }
}

// ============================================================
// テスト (変更不要)
// ============================================================

// --- 復習 ---
#[test]
fn test_r01_average() {
    assert!((average(&[1, 2, 3, 4, 5]).unwrap() - 3.0).abs() < 1e-9);
}
#[test]
fn test_r01_empty() {
    assert_eq!(average(&[]), None);
}

#[test]
fn test_r02_uppercase() {
    let mut words = vec![String::from("hello"), String::from("world")];
    uppercase_all(&mut words);
    assert_eq!(words, vec!["HELLO", "WORLD"]);
}

#[test]
fn test_r03_last_word() {
    assert_eq!(last_word("hello world foo"), "foo");
    assert_eq!(last_word("hello"), "hello");
}

// --- Q01 ---
#[test]
fn test_q01_area() {
    let r = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert!((r.area() - 12.0).abs() < 1e-9);
}
#[test]
fn test_q01_perimeter() {
    let r = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert!((r.perimeter() - 14.0).abs() < 1e-9);
}
#[test]
fn test_q01_square() {
    let r1 = Rectangle {
        width: 5.0,
        height: 5.0,
    };
    let r2 = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert!(r1.is_square());
    assert!(!r2.is_square());
}

// --- Q02 ---
#[test]
fn test_q02_new() {
    let c = Circle::new(5.0);
    assert!((c.radius - 5.0).abs() < 1e-9);
}
#[test]
fn test_q02_area() {
    let c = Circle::new(1.0);
    assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
}
#[test]
fn test_q02_larger() {
    let c1 = Circle::new(3.0);
    let c2 = Circle::new(5.0);
    assert!((Circle::larger(&c1, &c2).radius - 5.0).abs() < 1e-9);
}

// --- Q03 ---
#[test]
fn test_q03_new() {
    let p = Point3D::new(1.0, 2.0, 3.0);
    assert!((p.x - 1.0).abs() < 1e-9);
    assert!((p.y - 2.0).abs() < 1e-9);
    assert!((p.z - 3.0).abs() < 1e-9);
}
#[test]
fn test_q03_with_z() {
    let p1 = Point3D::new(1.0, 2.0, 3.0);
    let p2 = p1.with_z(99.0);
    assert!((p2.x - 1.0).abs() < 1e-9);
    assert!((p2.y - 2.0).abs() < 1e-9);
    assert!((p2.z - 99.0).abs() < 1e-9);
}

// --- Q04 ---
#[test]
fn test_q04_circle_area() {
    let s = Shape::Circle(1.0);
    assert!((s.area() - std::f64::consts::PI).abs() < 1e-9);
}
#[test]
fn test_q04_rect_area() {
    let s = Shape::Rectangle(3.0, 4.0);
    assert!((s.area() - 12.0).abs() < 1e-9);
}
#[test]
fn test_q04_triangle_area() {
    let s = Shape::Triangle(6.0, 4.0);
    assert!((s.area() - 12.0).abs() < 1e-9);
}

// --- Q05 ---
#[test]
fn test_q05_quit() {
    assert_eq!(Message::Quit.describe(), "Quit");
}
#[test]
fn test_q05_move() {
    assert_eq!(Message::Move { x: 1, y: 2 }.describe(), "Move to (1, 2)");
}
#[test]
fn test_q05_write() {
    assert_eq!(Message::Write(String::from("hi")).describe(), "Write: hi");
}
#[test]
fn test_q05_color() {
    assert_eq!(
        Message::ChangeColor(255, 0, 0).describe(),
        "Color: (255, 0, 0)"
    );
}

// --- Q06 ---
#[test]
fn test_q06_sum() {
    assert_eq!(sum_options(vec![Some(1), None, Some(3), None, Some(5)]), 9);
}
#[test]
fn test_q06_all_none() {
    assert_eq!(sum_options(vec![None, None]), 0);
}
#[test]
fn test_q06_empty() {
    assert_eq!(sum_options(vec![]), 0);
}

// --- Q07 ---
#[test]
fn test_q07_pop() {
    let mut stack = vec![5, 3, 0, 8, 1];
    let result = pop_until_zero(&mut stack);
    assert_eq!(result, vec![1, 8]);
}
#[test]
fn test_q07_no_zero() {
    let mut stack = vec![3, 2, 1];
    let result = pop_until_zero(&mut stack);
    assert_eq!(result, vec![1, 2, 3]);
}
#[test]
fn test_q07_zero_first() {
    let mut stack = vec![3, 2, 0];
    let result = pop_until_zero(&mut stack);
    assert_eq!(result, vec![]);
}

// --- Q08 ---
#[test]
fn test_q08_classify() {
    assert_eq!(classify(-5), "negative");
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(2), "small");
    assert_eq!(classify(7), "medium");
    assert_eq!(classify(12), "large even");
    assert_eq!(classify(11), "large odd");
}

// --- Q09 ---
#[test]
fn test_q09_new() {
    let c = Color::new(10, 20, 30);
    assert_eq!(c.to_tuple(), (10, 20, 30));
}
#[test]
fn test_q09_mix() {
    let c1 = Color::new(100, 200, 50);
    let c2 = Color::new(50, 100, 150);
    let mixed = c1.mix(&c2);
    assert_eq!(mixed.to_tuple(), (75, 150, 100));
}

// --- Q10 ---
#[test]
fn test_q10_num() {
    let e = Expr::Num(42.0);
    assert_eq!(e.eval(), Some(42.0));
}
#[test]
fn test_q10_add() {
    let e = Expr::Add(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(2.0)));
    assert_eq!(e.eval(), Some(3.0));
}
#[test]
fn test_q10_nested() {
    // (3 + 4) * 2
    let e = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Num(3.0)),
            Box::new(Expr::Num(4.0)),
        )),
        Box::new(Expr::Num(2.0)),
    );
    assert_eq!(e.eval(), Some(14.0));
}
#[test]
fn test_q10_div_zero() {
    let e = Expr::Div(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(0.0)));
    assert_eq!(e.eval(), None);
}
