// ============================================================
// Rust学習 フェーズ1: 基本型・関数・制御構造
// ============================================================
// 実行方法:
//   cargo test --test phase1_basics          # このファイル全体
//   cargo test --test phase1_basics test_q01 # Q01だけ
// ============================================================

// ============================================================
// Q01: 変数と基本型
// ============================================================
// Rustの変数はデフォルトでイミュータブル (Scalaの val に相当)
// mut をつけるとミュータブル (Scalaの var に相当)
//
// 【問題】celsius を摂氏から華氏に変換して返してください。
// 公式: fahrenheit = celsius * 9.0 / 5.0 + 32.0

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

// ============================================================
// Q02: 複数の戻り値 (タプル)
// ============================================================
// Rustのタプルは Scala と同様。(i32, i32) のように型を書く。
//
// 【問題】整数を受け取り、(商, 余り) のタプルを返してください。
// 例: divide(10, 3) => (3, 1)

fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

// ============================================================
// Q03: if 式
// ============================================================
// Rustの if は式 (Scala と同様)。値を返せる。
//
// 【問題】数値を受け取り、
//   正 => "positive" / 負 => "negative" / 0 => "zero"
// を返してください。

fn classify_number(n: i32) -> &'static str {
    if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    }
}

// ============================================================
// Q04: for ループ
// ============================================================
// ヒント: for i in 1..=n { ... }
//         Scalaの (1 to n) に相当
//
// 【問題】1からnまでの合計を返してください。

fn sum_to_n(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

// ============================================================
// Q05: Vec (可変長配列)
// ============================================================
// Scalaの ArrayBuffer に相当。
// Vec::new() で空作成、.push(x) で追加。
// vec![1, 2, 3] リテラルも使える。
//
// 【問題】1からnまでの整数を格納したVecを返してください。
// 例: range_vec(5) => vec![1, 2, 3, 4, 5]

fn range_vec(n: i32) -> Vec<i32> {
    (1..=n).collect()
}

// ============================================================
// Q06: Vecのイテレーション
// ============================================================
// 【問題】Vec<i32> を受け取り、偶数だけを集めたVecを返してください。
// まずは for ループで実装してみましょう。
// ヒント: &[i32] はVecのスライス（参照）。numbers.iter() も使えます。

fn filter_even(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &n in numbers {
        if n % 2 == 0 {
            result.push(n);
        }
    }
    result
}

// ============================================================
// Q07: String と &str
// ============================================================
// Rustの文字列は2種類あります:
//   &str   ... 文字列スライス (イミュータブルな参照、関数引数に多用)
//   String ... ヒープ上の文字列 (Scalaの String に近い、所有権あり)
//
// 【問題】名前を受け取り "Hello, {名前}!" という String を返してください。
// ヒント: format!("Hello, {}!", name) マクロが使えます。

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// ============================================================
// Q08: match 式
// ============================================================
// Scalaの match/case に相当。全ケース網羅が必須 (_ でワイルドカード)。
//
// 【問題】曜日番号(1〜7)を受け取り、曜日名を返してください。
//   1=>"Monday", 2=>"Tuesday", 3=>"Wednesday", 4=>"Thursday",
//   5=>"Friday", 6=>"Saturday", 7=>"Sunday", それ以外=>"Unknown"

fn day_name(day: u32) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Unknown",
    }
}

// ============================================================
// Q09: 再帰関数
// ============================================================
// 【問題】フィボナッチ数列のn番目の値を再帰で返してください。
//   fib(0) = 0, fib(1) = 1, fib(n) = fib(n-1) + fib(n-2)

fn fib(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

// ============================================================
// Q10: クロージャ (Closure)
// ============================================================
// Scalaのラムダ式に相当。
//   Scala: (x: Int) => x + 1
//   Rust:  |x: i32| x + 1  (型推論で |x| x + 1 も可)
//
// 【問題】クロージャ f と値 x を受け取り、f を x に2回適用した結果を返してください。
// 例: apply_twice(|x| x + 3, 7) => 13  (7+3=10, 10+3=13)

fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

// ============================================================
// Q11: Option 型
// ============================================================
// Scalaの Option[T] に相当。Some(値) または None。
// match や if let で中身を取り出す。
//
// 【問題】スライスの最大値を返してください。空なら None。

fn max_value(numbers: &[i32]) -> Option<i32> {
    numbers.iter().max().copied()
}

// ============================================================
// Q12: 総合問題 - FizzBuzz
// ============================================================
// 【問題】1からnまでのFizzBuzzをVec<String>で返してください。
//   15の倍数 => "FizzBuzz"  (先に判定!)
//    3の倍数 => "Fizz"
//    5の倍数 => "Buzz"
//   それ以外 => 数字の文字列 ("1", "2", ...)
// ヒント: 数値をStringに変換: n.to_string()

fn fizzbuzz(n: u32) -> Vec<String> {
    let mut result = Vec::new();
    for i in 1..=n {
        if i % 15 == 0 {
            result.push("FizzBuzz".to_string())
        } else if i % 3 == 0 {
            result.push("Fizz".to_string())
        } else if i % 5 == 0 {
            result.push("Buzz".to_string())
        } else {
            result.push(i.to_string())
        }
    }
    result
}

// ============================================================
// テスト (変更不要)
// ============================================================
#[test]
fn test_q01_freezing() {
    assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-9);
}
#[test]
fn test_q01_boiling() {
    assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-9);
}
#[test]
fn test_q01_body_temp() {
    assert!((celsius_to_fahrenheit(37.0) - 98.6).abs() < 0.01);
}

#[test]
fn test_q02_basic() {
    assert_eq!(divide(10, 3), (3, 1));
}
#[test]
fn test_q02_even() {
    assert_eq!(divide(20, 4), (5, 0));
}
#[test]
fn test_q02_small() {
    assert_eq!(divide(1, 5), (0, 1));
}

#[test]
fn test_q03_positive() {
    assert_eq!(classify_number(42), "positive");
}
#[test]
fn test_q03_negative() {
    assert_eq!(classify_number(-7), "negative");
}
#[test]
fn test_q03_zero() {
    assert_eq!(classify_number(0), "zero");
}

#[test]
fn test_q04_basic() {
    assert_eq!(sum_to_n(5), 15);
}
#[test]
fn test_q04_one() {
    assert_eq!(sum_to_n(1), 1);
}
#[test]
fn test_q04_hundred() {
    assert_eq!(sum_to_n(100), 5050);
}

#[test]
fn test_q05_five() {
    assert_eq!(range_vec(5), vec![1, 2, 3, 4, 5]);
}
#[test]
fn test_q05_one() {
    assert_eq!(range_vec(1), vec![1]);
}
#[test]
fn test_q05_empty() {
    assert_eq!(range_vec(0), vec![]);
}

#[test]
fn test_q06_basic() {
    assert_eq!(filter_even(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
}
#[test]
fn test_q06_all_odd() {
    assert_eq!(filter_even(&[1, 3, 5]), vec![]);
}
#[test]
fn test_q06_empty() {
    assert_eq!(filter_even(&[]), vec![]);
}

#[test]
fn test_q07_basic() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
}
#[test]
fn test_q07_japanese() {
    assert_eq!(greet("太郎"), "Hello, 太郎!");
}

#[test]
fn test_q08_monday() {
    assert_eq!(day_name(1), "Monday");
}
#[test]
fn test_q08_wednesday() {
    assert_eq!(day_name(3), "Wednesday");
}
#[test]
fn test_q08_sunday() {
    assert_eq!(day_name(7), "Sunday");
}
#[test]
fn test_q08_unknown() {
    assert_eq!(day_name(8), "Unknown");
}

#[test]
fn test_q09_base_cases() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
}
#[test]
fn test_q09_fib_10() {
    assert_eq!(fib(10), 55);
}
#[test]
fn test_q09_fib_20() {
    assert_eq!(fib(20), 6765);
}

#[test]
fn test_q10_add3() {
    assert_eq!(apply_twice(|x| x + 3, 7), 13);
}
#[test]
fn test_q10_double() {
    assert_eq!(apply_twice(|x| x * 2, 5), 20);
}

#[test]
fn test_q11_basic() {
    assert_eq!(max_value(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(9));
}
#[test]
fn test_q11_single() {
    assert_eq!(max_value(&[42]), Some(42));
}
#[test]
fn test_q11_empty() {
    assert_eq!(max_value(&[]), None);
}

#[test]
fn test_q12_values() {
    let result = fizzbuzz(15);
    assert_eq!(result[0], "1");
    assert_eq!(result[2], "Fizz");
    assert_eq!(result[4], "Buzz");
    assert_eq!(result[14], "FizzBuzz");
}
#[test]
fn test_q12_length() {
    assert_eq!(fizzbuzz(10).len(), 10);
}
