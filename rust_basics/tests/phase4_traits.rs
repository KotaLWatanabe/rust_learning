// ============================================================
// Rust学習 フェーズ4: トレイト・ジェネリクス
// ============================================================
// 実行方法:
//   cargo test --test phase4_traits          # このファイル全体
//   cargo test --test phase4_traits test_r01 # 復習だけ
//   cargo test --test phase4_traits test_q01 # 新問Q1だけ
//
// Scalaとの対応:
//   trait          ≒ trait (ほぼ同じ！)
//   impl Trait for ≒ extends / with
//   T: Trait        ≒ T <: Trait (型境界)
//   ジェネリクス    ≒ 型パラメータ [T]
//   derive          ≒ @derive アノテーション (Haskell由来)
// ============================================================

// ============================================================
// 【フェーズ3 復習】
// ============================================================

// R01: struct + impl
// 【問題】以下の Stack<i32> 構造体を実装してください。
//   - push(n): 値を追加
//   - pop(): 値を取り出す (空なら None)
//   - is_empty(): 空かどうか

extern crate core;

struct Stack {
    data: Vec<i32>,
}

impl Stack {
    fn new() -> Stack {
        Self { data: Vec::new() }
    }
    fn push(&mut self, n: i32) {
        self.data.push(n)
    }
    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// R02: enum + match
// 【問題】以下の Traffic Light の次の状態を返す関数を実装してください。
//   Red => Green => Yellow => Red => ...

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
    fn duration_secs(&self) -> u32 {
        // Red=60, Yellow=5, Green=45
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

// R03: Option メソッド
// 【問題】Vec<Option<f64>> を受け取り、
// Some の値の平均を返してください。Someが0個なら None。

fn average_options(values: &[Option<f64>]) -> Option<f64> {
    let numbers: Vec<f64> = values.iter().copied().flatten().collect();
    if numbers.is_empty() {
        None
    } else {
        Some(numbers.iter().fold(0.0, |acc, x| acc + *x) / numbers.len() as f64)
    }
}

// ============================================================
// 【フェーズ4】トレイト・ジェネリクス
// ============================================================

// ============================================================
// Q01: トレイトの定義と実装
// ============================================================
// Rustのトレイトは Scala のトレイトとほぼ同じ概念。
// ただし Rust では継承はなく、コンポジションで組み合わせる。
//
// 定義:
//   trait Greet {
//       fn greet(&self) -> String;
//       fn shout(&self) -> String {  // デフォルト実装
//           self.greet().to_uppercase()
//       }
//   }
//
// 実装:
//   impl Greet for SomeStruct { ... }
//
// 【問題】以下の Describable トレイトを 3つの型に実装してください。

trait Describable {
    fn describe(&self) -> String;

    // デフォルト実装: describe() の結果を大文字で返す
    fn shout(&self) -> String {
        self.describe().to_uppercase()
    }
}

struct Dog {
    name: String,
    breed: String,
}

struct Book {
    title: String,
    author: String,
    pages: u32,
}

struct Temperature {
    celsius: f64,
}

// 【問題1】Dog の Describable を実装 (例: "Buddy is a Labrador")
impl Describable for Dog {
    fn describe(&self) -> String {
        format!("{} is a {}", self.name, self.breed)
    }
}

// 【問題2】Book の Describable を実装 (例: "Rust Programming by Steve, 526 pages")
impl Describable for Book {
    fn describe(&self) -> String {
        format!("{} by {}, {} pages", self.title, self.author, self.pages)
    }
}

// 【問題3】Temperature の Describable を実装 (例: "25.0°C (77.0°F)")
impl Describable for Temperature {
    fn describe(&self) -> String {
        let fahrenheit = self.celsius * 9.0 / 5.0 + 32.0;
        format!("{:.1}°C ({:.1}°F)", self.celsius, fahrenheit)
    }
}

// ============================================================
// Q02: 標準トレイト - Display と Debug
// ============================================================
// Rust には標準で多くのトレイトが用意されている。
//
//   Display ... println!("{}", x)   で使われる (人向け表示)
//   Debug   ... println!("{:?}", x) で使われる (開発者向け)
//
// Scala の toString() に相当するのが Display。
// Debug は #[derive(Debug)] で自動生成できることが多い。
//
// 【問題】Matrix2x2 構造体に Display を実装してください。
// 表示形式:
//   | 1 2 |
//   | 3 4 |

use std::collections::HashSet;
use std::fmt;

#[derive(Debug)] // Debug は自動導出
struct Matrix2x2 {
    data: [[f64; 2]; 2], // 2x2の二次元配列
}

impl Matrix2x2 {
    fn new(a: f64, b: f64, c: f64, d: f64) -> Matrix2x2 {
        Matrix2x2 {
            data: [[a, b], [c, d]],
        }
    }
}

impl fmt::Display for Matrix2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ヒント: write!(f, "| {} {} |\n| {} {} |", ...)
        write!(
            f,
            "| {} {} |\n| {} {} |",
            self.data[0][0], self.data[0][1], self.data[1][0], self.data[1][1]
        )
    }
}

// ============================================================
// Q03: 標準トレイト - PartialOrd / PartialEq
// ============================================================
// Rust では == や < などの演算子もトレイトで定義される。
//   PartialEq  ... == と != (Scala の equals に相当)
//   PartialOrd ... < > <= >= (Scala の Ordered に相当)
//
// プリミティブ型や多くの標準型は derive で自動導出できる。
//
// 【問題】Version 構造体に PartialEq と PartialOrd を実装してください。
// バージョン比較: major > minor > patch の優先順位

#[derive(Debug)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn new(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.major, self.minor, self.patch).partial_cmp(&(other.major, other.minor, other.patch))
    }
}

// ============================================================
// Q04: ジェネリクス関数
// ============================================================
// Scala の型パラメータ [T] に相当。
// Rust では型境界 (T: Trait) をコロンで書く。
//
//   fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }
//
// Scala: def largest[T <: Ordered[T]](list: Seq[T]): T
//
// 【問題】スライスの最大値を返すジェネリクス関数を実装してください。
// T は比較可能な型に限定する。

fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let first = list.first()?;
    Some(
        list.iter()
            .fold(first, |max, x| if *max < *x { x } else { max }),
    )
}

// ============================================================
// Q05: ジェネリクス構造体
// ============================================================
// 構造体もジェネリクスにできる。
//
// 【問題】任意の型を2つ保持する Pair<T> 構造体を実装してください。
//   - new(first, second): 生成
//   - first() / second(): 各要素への参照を返す
//   - swap(): 順序を入れ替えた新しい Pair を返す (T: Clone が必要)

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Self { first, second }
    }

    fn first(&self) -> &T {
        &self.first
    }

    fn second(&self) -> &T {
        &self.second
    }
}

impl<T: Clone> Pair<T> {
    fn swap(&self) -> Pair<T> {
        Pair {
            first: self.second.clone(),
            second: self.first.clone(),
        }
    }
}

// ============================================================
// Q06: トレイト境界 (複数)
// ============================================================
// 複数のトレイト境界を指定できる。
//   T: Display + PartialOrd
//
// where 句で読みやすく書くこともできる:
//   fn foo<T>(x: T) where T: Display + PartialOrd { ... }
//
// 【問題】2つの値を受け取り、大きい方を "{} wins over {}" 形式で
// 表示した String を返す関数を実装してください。
// 同じ場合は "{} ties with {}" とする。

fn compare_and_describe<T>(a: T, b: T) -> String
where
    T: PartialOrd + fmt::Display,
{
    if a == b {
        format!("{} ties with {}", a, b)
    } else {
        let (max, min) = if a > b { (a, b) } else { (b, a) };
        format!("{} wins over {}", max, min)
    }
}

// ============================================================
// Q07: トレイトオブジェクト (dyn Trait)
// ============================================================
// ジェネリクスはコンパイル時に型が決まる (静的ディスパッチ)。
// dyn Trait は実行時に型が決まる (動的ディスパッチ)。
//
// Scala の trait 参照と同じ概念:
//   Scala: val animals: List[Animal] = List(dog, cat)
//   Rust:  let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];
//
// 【問題】以下の Animal トレイトを実装し、
// 動物のリストから全ての鳴き声を集める関数を実装してください。

trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    fn info(&self) -> String {
        format!("{} says {}", self.name(), self.sound())
    }
}

struct Cat {
    name: String,
}
struct Cow {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    fn sound(&self) -> &str {
        "woof"
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    fn sound(&self) -> &str {
        "meow"
    }
}

impl Animal for Cow {
    fn name(&self) -> &str {
        &self.name
    }
    fn sound(&self) -> &str {
        "moo"
    }
}

// 【問題】動物リストから全ての info() を集めた Vec<String> を返してください。
fn all_sounds(animals: &[Box<dyn Animal>]) -> Vec<String> {
    animals.iter().map(|a| a.info()).collect()
}

// ============================================================
// Q08: derive マクロ
// ============================================================
// #[derive(...)] で標準トレイトを自動実装できる。
// よく使うもの:
//   Debug   ... {:?} でデバッグ表示
//   Clone   ... .clone() で複製
//   PartialEq ... == で比較
//   Hash    ... HashMap のキーにできる
//
// 【問題】以下の Card 構造体に必要な derive を追加し、
// 手札(Vec<Card>)から重複を除いた新しいVecを返す関数を実装してください。
// ヒント: 重複除去には HashSet が使える

#[derive(Debug, Hash, Eq, PartialEq, Clone)] // ← 必要な derive を追加してください
struct Card {
    suit: String, // "Hearts", "Diamonds", "Clubs", "Spades"
    value: u8,    // 1-13
}

impl Card {
    fn new(suit: &str, value: u8) -> Card {
        Card {
            suit: suit.to_string(),
            value,
        }
    }
}

fn dedup_cards(cards: Vec<Card>) -> Vec<Card> {
    let set: HashSet<Card> = cards.into_iter().collect();
    set.into_iter().collect()
}

// ============================================================
// Q09: impl Trait (引数・戻り値)
// ============================================================
// Rust 2018 以降、impl Trait 構文でより簡潔に書ける。
//
// 引数: fn foo(x: impl Display)  =  fn foo<T: Display>(x: T)
// 戻り値: fn make_adder(n: i32) -> impl Fn(i32) -> i32
//          ★ クロージャを返す場合は impl Fn が必要
//             (クロージャの具体的な型名は書けないため)
//
// 【問題】n を受け取り、「n を足すクロージャ」を返す関数を実装してください。
// 例: let add5 = make_adder(5); add5(3) => 8

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // ヒント: move |x| x + n
    //   move キーワード: クロージャが n の所有権を取得する
    move |x| x + n
}

// ============================================================
// Q10: 総合問題 - ジェネリックなキャッシュ
// ============================================================
// 【問題】計算結果をキャッシュする構造体 Cacher を実装してください。
// - 初回呼び出し時は computation を実行してキャッシュする
// - 2回目以降はキャッシュした値を返す
// - Scala の lazy val に似た概念

struct Cacher<T, F>
where
    F: Fn(T) -> T,
    T: Copy,
{
    computation: F,
    value: Option<T>,
}

impl<T, F> Cacher<T, F>
where
    F: Fn(T) -> T,
    T: Copy,
{
    fn new(computation: F) -> Cacher<T, F> {
        Self {
            computation,
            value: None,
        }
    }

    // 初回は computation(arg) を実行してキャッシュ、以降はキャッシュを返す
    fn value(&mut self, arg: T) -> T {
        match self.value {
            None => {
                let value: T = (self.computation)(arg);
                self.value = Some(value);
                value
            }
            Some(value) => value,
        }
    }
}

// ============================================================
// テスト (変更不要)
// ============================================================

// --- 復習 ---
#[test]
fn test_r01_stack() {
    let mut s = Stack::new();
    assert!(s.is_empty());
    s.push(1);
    s.push(2);
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}

#[test]
fn test_r02_traffic() {
    let t = TrafficLight::Red;
    assert_eq!(t.duration_secs(), 60);
    // Red -> Green
    let t2 = t.next();
    assert_eq!(t2.duration_secs(), 45);
}

#[test]
fn test_r03_average_options() {
    let v = vec![Some(1.0), None, Some(3.0), None, Some(5.0)];
    assert!((average_options(&v).unwrap() - 3.0).abs() < 1e-9);
    assert_eq!(average_options(&[None, None]), None);
}

// --- Q01 ---
#[test]
fn test_q01_dog() {
    let d = Dog {
        name: "Buddy".to_string(),
        breed: "Labrador".to_string(),
    };
    assert_eq!(d.describe(), "Buddy is a Labrador");
    assert_eq!(d.shout(), "BUDDY IS A LABRADOR");
}
#[test]
fn test_q01_book() {
    let b = Book {
        title: "Rust Programming".to_string(),
        author: "Steve".to_string(),
        pages: 526,
    };
    assert_eq!(b.describe(), "Rust Programming by Steve, 526 pages");
}
#[test]
fn test_q01_temperature() {
    let t = Temperature { celsius: 25.0 };
    assert_eq!(t.describe(), "25.0°C (77.0°F)");
}

// --- Q02 ---
#[test]
fn test_q02_display() {
    let m = Matrix2x2::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(format!("{}", m), "| 1 2 |\n| 3 4 |");
}

// --- Q03 ---
#[test]
fn test_q03_eq() {
    let v1 = Version::new(1, 2, 3);
    let v2 = Version::new(1, 2, 3);
    let v3 = Version::new(1, 2, 4);
    assert!(v1 == v2);
    assert!(v1 != v3);
}
#[test]
fn test_q03_ord() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(1, 1, 0);
    assert!(v1 < v2);
    assert!(v1 < v3);
    assert!(v2 > v3);
}

// --- Q04 ---
#[test]
fn test_q04_largest_i32() {
    assert_eq!(largest(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(&9));
}
#[test]
fn test_q04_largest_f64() {
    assert_eq!(largest(&[1.5, 3.2, 2.7]), Some(&3.2));
}
#[test]
fn test_q04_empty() {
    assert_eq!(largest::<i32>(&[]), None);
}

// --- Q05 ---
#[test]
fn test_q05_pair() {
    let p = Pair::new(1, 2);
    assert_eq!(p.first(), &1);
    assert_eq!(p.second(), &2);
}
#[test]
fn test_q05_swap() {
    let p = Pair::new("hello", "world");
    let swapped = p.swap();
    assert_eq!(swapped.first(), &"world");
    assert_eq!(swapped.second(), &"hello");
}

// --- Q06 ---
#[test]
fn test_q06_compare() {
    assert_eq!(compare_and_describe(5, 3), "5 wins over 3");
    assert_eq!(compare_and_describe(2, 9), "9 wins over 2");
    assert_eq!(compare_and_describe(4, 4), "4 ties with 4");
}

// --- Q07 ---
#[test]
fn test_q07_sounds() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: "Rex".to_string(),
            breed: "GSD".to_string(),
        }),
        Box::new(Cat {
            name: "Whiskers".to_string(),
        }),
        Box::new(Cow {
            name: "Bessie".to_string(),
        }),
    ];
    let sounds = all_sounds(&animals);
    assert_eq!(sounds[0], "Rex says woof");
    assert_eq!(sounds[1], "Whiskers says meow");
    assert_eq!(sounds[2], "Bessie says moo");
}

// --- Q08 ---
#[test]
fn test_q08_dedup() {
    let cards = vec![
        Card::new("Hearts", 1),
        Card::new("Hearts", 1), // 重複
        Card::new("Spades", 13),
    ];
    let result = dedup_cards(cards);
    assert_eq!(result.len(), 2);
}

// --- Q09 ---
#[test]
fn test_q09_adder() {
    let add5 = make_adder(5);
    assert_eq!(add5(3), 8);
    assert_eq!(add5(10), 15);
}
#[test]
fn test_q09_adder_zero() {
    let add0 = make_adder(0);
    assert_eq!(add0(42), 42);
}

// --- Q10 ---
#[test]
fn test_q10_cacher() {
    // let mut _call_count = 0;
    // 注: クロージャ内でcall_countを変更するにはCell等が必要なため
    // ここではシンプルに計算結果のキャッシュのみ検証
    let mut c = Cacher::new(|x: i32| x * 2);
    assert_eq!(c.value(5), 10);
    assert_eq!(c.value(5), 10); // キャッシュから返る
}
#[test]
fn test_q10_cacher_f64() {
    let mut c = Cacher::new(|x: f64| x * x);
    assert!((c.value(3.0) - 9.0).abs() < 1e-9);
    assert!((c.value(3.0) - 9.0).abs() < 1e-9);
}
