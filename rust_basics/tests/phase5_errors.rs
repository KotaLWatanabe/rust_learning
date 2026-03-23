// ============================================================
// Rust学習 フェーズ5: エラーハンドリング
// ============================================================
// 実行方法:
//   cargo test --test phase5_errors          # このファイル全体
//   cargo test --test phase5_errors test_r01 # 復習だけ
//   cargo test --test phase5_errors test_q01 # 新問Q1だけ
//
// Scalaとの対応:
//   Result<T, E>  ≒ Either[E, T]  (Ok=Right, Err=Left)
//   ?演算子       ≒ flatMap / for-comprehension のエラー伝播
//   panic!()      ≒ throw Exception (ただし回復不能)
//   カスタムエラー ≒ sealed trait + case class のエラー階層
// ============================================================

// ============================================================
// 【フェーズ4 復習】
// ============================================================

// R01: トレイトの定義と実装
// 【問題】Area トレイトを定義し、Square と RightTriangle に実装してください。
//   Square: 一辺の長さから面積 (side * side)
//   RightTriangle: 2辺の長さから面積 (a * b / 2.0)

trait Area {
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
}

struct RightTriangle {
    a: f64,
    b: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Area for RightTriangle {
    fn area(&self) -> f64 {
        self.a * self.b / 2.0
    }
}

// R02: ジェネリクス関数
// 【問題】2つの値を受け取り、小さい方を返すジェネリクス関数を実装してください。

fn smaller<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

// R03: dyn Trait
// 【問題】Vec<Box<dyn Area>> を受け取り、面積の合計を返してください。

fn total_area(shapes: &[Box<dyn Area>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// ============================================================
// 【フェーズ5】エラーハンドリング
// ============================================================
//
// ★ Rustのエラーハンドリング哲学
//
//   panic!()     ... 回復不能なエラー (バグ、assertion失敗など)
//                    Scalaの throw に近いが、基本的に使わない
//
//   Result<T, E> ... 回復可能なエラー (ファイルなし、パース失敗など)
//                    Scalaの Either[E, T] に相当
//                    Ok(value) = 成功  /  Err(error) = 失敗
//
// Rustでは例外機構がないため、失敗する可能性のある処理は
// 必ず Result を返す。これがコンパイラで強制される。
// ============================================================

// ============================================================
// Q01: Result の基本
// ============================================================
// Result<T, E> は Ok(T) または Err(E) のいずれか。
//
// match で取り出す:
//   match result {
//       Ok(v)  => println!("成功: {}", v),
//       Err(e) => println!("失敗: {}", e),
//   }
//
// 【問題】文字列を i32 にパースする関数を実装してください。
// 成功したら Ok(数値)、失敗したら Err("parse error: {元の文字列}") を返す。
// 例: parse_int("42")  => Ok(42)
//     parse_int("abc") => Err("parse error: abc")

fn parse_int(s: &str) -> Result<i32, String> {
    // ヒント: s.parse::<i32>() は Result<i32, ParseIntError> を返す
    //         .map_err(|_| format!("parse error: {}", s)) でエラーを変換できる
    s.parse::<i32>().map_err(|_| format!("parse error: {}", s))
}

// ============================================================
// Q02: ? 演算子 (エラー伝播)
// ============================================================
// ? は Result や Option に対して使える糖衣構文。
//   - Ok(v)  なら v を取り出して続行
//   - Err(e) なら即座に return Err(e)
//
// Scala の for-comprehension のエラー伝播に相当:
//   Scala: for { a <- parseA; b <- parseB(a) } yield result
//   Rust:  let a = parse_a()?; let b = parse_b(a)?; Ok(result)
//
// 【問題】"x,y" 形式の文字列を受け取り、2つの i32 の合計を返してください。
// カンマがない、または数値でない場合は Err を返す。
// 例: add_csv("10,20") => Ok(30)
//     add_csv("10,xx") => Err(...)
//     add_csv("10")    => Err(...)

fn add_csv(s: &str) -> Result<i32, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err("parse error".into());
    }
    let a = parse_int(parts[0])?;
    let b = parse_int(parts[1])?;
    Ok(a + b)
}

// ============================================================
// Q03: Result のメソッド
// ============================================================
// Result には便利なメソッドが多数ある (Option と対称的):
//
//   .map(|v| ...)         Ok の中身を変換
//   .map_err(|e| ...)     Err の中身を変換
//   .and_then(|v| f(v))   Ok なら次の Result を返す (flatMap)
//   .unwrap_or(default)   Err なら default を使う
//   .ok()                 Result<T,E> を Option<T> に変換
//   .is_ok() / .is_err()
//
// 【問題1】文字列を受け取り、パースして2倍にした結果を返してください。
// 例: parse_and_double("5") => Ok(10)
//     parse_and_double("x") => Err(...)

fn parse_and_double(s: &str) -> Result<i32, String> {
    parse_int(s).map(|n| n * 2)
}

// 【問題2】文字列スライスを受け取り、全て i32 にパースした Vec を返してください。
// 1つでも失敗したら Err を返す。
// 例: parse_all(&["1", "2", "3"]) => Ok(vec![1, 2, 3])
//     parse_all(&["1", "x", "3"]) => Err(...)

fn parse_all(strings: &[&str]) -> Result<Vec<i32>, String> {
    let mut result = Vec::new();
    for &s in strings {
        result.push(parse_int(s)?);
    }
    Ok(result)
}

// ============================================================
// Q04: カスタムエラー型
// ============================================================
// 本格的なアプリでは独自のエラー型を定義する。
// Scala の sealed trait + case class のエラー階層に相当。
//
// 慣例として std::error::Error トレイトを実装する。
// Display (人向けメッセージ) も必要。
//
// 【問題】以下の AppError enum を完成させてください。

#[derive(Debug, PartialEq)]
enum AppError {
    ParseError(String),                            // 数値パース失敗
    DivisionByZero,                                // ゼロ除算
    NegativeInput(i32),                            // 負の数は不正
    OutOfRange { value: i32, min: i32, max: i32 }, // 範囲外
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 各バリアントを人間が読める形式で表示:
        //   ParseError(s)        => "Failed to parse: {s}"
        //   DivisionByZero       => "Division by zero"
        //   NegativeInput(n)     => "Negative input: {n}"
        //   OutOfRange{v,min,max}=> "Value {v} out of range [{min}, {max}]"
        match self {
            AppError::ParseError(s) => write!(f, "Failed to parse: {s}"),
            AppError::DivisionByZero => write!(f, "Division by zero"),
            AppError::NegativeInput(n) => write!(f, "Negative input: {n}"),
            AppError::OutOfRange { value, min, max } => {
                write!(f, "Value {value} out of range [{min}, {max}]")
            }
        }
    }
}

// ============================================================
// Q05: カスタムエラーを使う
// ============================================================
// 【問題】以下の関数を AppError を使って実装してください。

// 文字列をパースして正の整数として返す
// 失敗したら AppError::ParseError、負なら AppError::NegativeInput
fn parse_positive(s: &str) -> Result<i32, AppError> {
    parse_int(s)
        .map_err(|_| AppError::ParseError(s.into()))
        .and_then(|n| {
            if n < 0 {
                Err(AppError::NegativeInput(n))
            } else {
                Ok(n)
            }
        })
}

// 2つの正の整数を受け取り、安全に割り算する
// ゼロ除算なら AppError::DivisionByZero
fn safe_divide(a: i32, b: i32) -> Result<i32, AppError> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err(AppError::DivisionByZero)
    }
}

// 値が [min, max] の範囲内かチェック
// 範囲外なら AppError::OutOfRange
fn check_range(value: i32, min: i32, max: i32) -> Result<i32, AppError> {
    if (min..=max).contains(&value) {
        Ok(value)
    } else {
        Err(AppError::OutOfRange { value, min, max })
    }
}

// ============================================================
// Q06: エラーの連鎖 (? 演算子 + カスタムエラー)
// ============================================================
// 【問題】文字列を受け取り、
//   1. i32 にパース (失敗 => ParseError)
//   2. 正の数か確認 (負 => NegativeInput)
//   3. 100以下か確認 (超過 => OutOfRange { value, min:1, max:100 })
// 全て通過したら Ok(値) を返す関数を実装してください。

fn validate_score(s: &str) -> Result<i32, AppError> {
    let int = parse_positive(s)?;
    check_range(int, 1, 100)
}

// ============================================================
// Q07: 複数のエラー型を扱う (Box<dyn Error>)
// ============================================================
// 異なるエラー型が混在する場合、Box<dyn Error> でまとめられる。
// (Scala の Exception の共通基底クラスに相当)
//
// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
//
// 【問題】以下の処理を1つの関数にまとめてください:
//   1. "key=value" 形式の文字列をパース
//   2. value 部分を i32 にパース
//   3. 値が 0..=255 の範囲かチェック (超過は AppError::OutOfRange)
// どちらのエラーも Box<dyn Error> として返す。

use std::{error::Error, fmt};

// AppError に Error トレイトを実装 (Display + Debug があれば可)
impl Error for AppError {}

fn parse_config_value(input: &str) -> Result<i32, Box<dyn Error>> {
    let parts: Vec<&str> = input.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err("invalid format".into());
    }
    let value: i32 = parts[1].parse()?;
    Ok(check_range(value, 0, 255)?)
}

// ============================================================
// Q08: unwrap / expect / パニックの使いどころ
// ============================================================
// unwrap() と expect() はエラー時に panic する。
//   .unwrap()         => panics with "called unwrap on Err"
//   .expect("msg")    => panics with "msg"
//
// 使っていい場面:
//   - テストコード内
//   - プログラムの起動時設定 (失敗 = バグ)
//   - 絶対に None/Err にならないと確信できる場合
//
// 【問題】以下の関数を実装してください。
// 設定値のデフォルトを持つパーサー。
// パースできた場合はその値、できない場合はデフォルト値を返す。
// (エラーを伝播させず、常に値を返す)

fn parse_with_default(s: &str, default: i32) -> i32 {
    parse_int(s).unwrap_or(default)
}

// ============================================================
// Q09: Result と Iterator の組み合わせ
// ============================================================
// Iterator に Result を絡めるパターンは頻出。
//
// collect::<Result<Vec<_>, _>>() を使うと
// イテレータの Result を1つの Result<Vec> に変換できる。
//
// 例:
//   let results: Result<Vec<i32>, _> =
//       vec!["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
//
// 【問題】文字列のスライスを受け取り、
// 全て i32 にパースして合計を返してください。
// 1つでも失敗したら Err を返す。
// ★ for ループでなく Iterator のメソッドチェーンで実装してください。

fn sum_strings(strings: &[&str]) -> Result<i32, String> {
    // ヒント:
    //   strings.iter()
    //       .map(|s| parse_int(s))
    //       .collect::<Result<Vec<_>, _>>()
    //       .map(|v| v.iter().sum())
    strings
        .iter()
        .map(|s| parse_int(s))
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.iter().sum())
}

// ============================================================
// Q10: 総合問題 - CSV 行パーサー
// ============================================================
// 【問題】以下の形式の CSV 行をパースする関数を実装してください。
// 形式: "名前,年齢,スコア"
//   - フィールドが3つでない  => AppError::ParseError
//   - 年齢が数値でない       => AppError::ParseError
//   - スコアが数値でない     => AppError::ParseError
//   - 年齢が負               => AppError::NegativeInput
//   - スコアが 0-100 範囲外  => AppError::OutOfRange
//
// 例: parse_csv_row("Alice,30,85")  => Ok(Student { name:"Alice", age:30, score:85 })
//     parse_csv_row("Bob,-1,85")    => Err(AppError::NegativeInput(-1))
//     parse_csv_row("Carol,25,150") => Err(AppError::OutOfRange { value:150, min:0, max:100 })

#[derive(Debug, PartialEq)]
struct Student {
    name: String,
    age: i32,
    score: i32,
}

fn parse_csv_row(row: &str) -> Result<Student, AppError> {
    let columns: Vec<&str> = row.splitn(3, ',').collect();
    if columns.len() != 3 {
        return Err(AppError::ParseError(row.into()));
    }
    let age = parse_positive(columns[1])?;
    let score = parse_int(columns[2]).map_err(|_| AppError::ParseError(columns[2].into()))?;

    if !(0..=100).contains(&score) {
        return Err(AppError::OutOfRange {
            value: score,
            min: 0,
            max: 100,
        });
    }
    Ok(Student {
        name: columns[0].into(),
        age,
        score,
    })
}

// ============================================================
// テスト (変更不要)
// ============================================================

// --- 復習 ---
#[test]
fn test_r01_area() {
    let s = Square { side: 4.0 };
    assert!((s.area() - 16.0).abs() < 1e-9);
    let t = RightTriangle { a: 3.0, b: 4.0 };
    assert!((t.area() - 6.0).abs() < 1e-9);
}

#[test]
fn test_r02_smaller() {
    assert_eq!(smaller(3, 7), 3);
    assert_eq!(smaller(3.14, 2.71), 2.71);
}

#[test]
fn test_r03_total_area() {
    let shapes: Vec<Box<dyn Area>> = vec![
        Box::new(Square { side: 3.0 }),
        Box::new(RightTriangle { a: 4.0, b: 6.0 }),
    ];
    assert!((total_area(&shapes) - 21.0).abs() < 1e-9);
}

// --- Q01 ---
#[test]
fn test_q01_ok() {
    assert_eq!(parse_int("42"), Ok(42));
    assert_eq!(parse_int("-7"), Ok(-7));
}
#[test]
fn test_q01_err() {
    assert_eq!(parse_int("abc"), Err("parse error: abc".to_string()));
}

// --- Q02 ---
#[test]
fn test_q02_ok() {
    assert_eq!(add_csv("10,20"), Ok(30));
}
#[test]
fn test_q02_no_comma() {
    assert!(add_csv("10").is_err());
}
#[test]
fn test_q02_invalid() {
    assert!(add_csv("10,xx").is_err());
}

// --- Q03 ---
#[test]
fn test_q03_double_ok() {
    assert_eq!(parse_and_double("5"), Ok(10));
}
#[test]
fn test_q03_double_err() {
    assert!(parse_and_double("x").is_err());
}
#[test]
fn test_q03_parse_all_ok() {
    assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
}
#[test]
fn test_q03_parse_all_err() {
    assert!(parse_all(&["1", "x", "3"]).is_err());
}

// --- Q04 ---
#[test]
fn test_q04_display() {
    assert_eq!(
        format!("{}", AppError::ParseError("foo".to_string())),
        "Failed to parse: foo"
    );
    assert_eq!(format!("{}", AppError::DivisionByZero), "Division by zero");
    assert_eq!(
        format!("{}", AppError::NegativeInput(-5)),
        "Negative input: -5"
    );
    assert_eq!(
        format!(
            "{}",
            AppError::OutOfRange {
                value: 200,
                min: 0,
                max: 100
            }
        ),
        "Value 200 out of range [0, 100]"
    );
}

// --- Q05 ---
#[test]
fn test_q05_parse_positive_ok() {
    assert_eq!(parse_positive("42"), Ok(42));
}
#[test]
fn test_q05_parse_positive_parse_err() {
    assert_eq!(
        parse_positive("abc"),
        Err(AppError::ParseError("abc".to_string()))
    );
}
#[test]
fn test_q05_parse_positive_negative() {
    assert_eq!(parse_positive("-3"), Err(AppError::NegativeInput(-3)));
}
#[test]
fn test_q05_safe_divide_ok() {
    assert_eq!(safe_divide(10, 2), Ok(5));
}
#[test]
fn test_q05_safe_divide_zero() {
    assert_eq!(safe_divide(10, 0), Err(AppError::DivisionByZero));
}
#[test]
fn test_q05_check_range_ok() {
    assert_eq!(check_range(50, 1, 100), Ok(50));
}
#[test]
fn test_q05_check_range_err() {
    assert_eq!(
        check_range(150, 1, 100),
        Err(AppError::OutOfRange {
            value: 150,
            min: 1,
            max: 100
        })
    );
}

// --- Q06 ---
#[test]
fn test_q06_ok() {
    assert_eq!(validate_score("85"), Ok(85));
}
#[test]
fn test_q06_parse_err() {
    assert!(matches!(
        validate_score("abc"),
        Err(AppError::ParseError(_))
    ));
}
#[test]
fn test_q06_negative() {
    assert_eq!(validate_score("-1"), Err(AppError::NegativeInput(-1)));
}
#[test]
fn test_q06_out_of_range() {
    assert_eq!(
        validate_score("150"),
        Err(AppError::OutOfRange {
            value: 150,
            min: 1,
            max: 100
        })
    );
}

// --- Q07 ---
#[test]
fn test_q07_ok() {
    let result = parse_config_value("timeout=30");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 30);
}
#[test]
fn test_q07_parse_err() {
    assert!(parse_config_value("timeout=abc").is_err());
}
#[test]
fn test_q07_range_err() {
    assert!(parse_config_value("timeout=999").is_err());
}

// --- Q08 ---
#[test]
fn test_q08_ok() {
    assert_eq!(parse_with_default("42", 0), 42);
}
#[test]
fn test_q08_default() {
    assert_eq!(parse_with_default("abc", 99), 99);
}

// --- Q09 ---
#[test]
fn test_q09_ok() {
    assert_eq!(sum_strings(&["1", "2", "3", "4"]), Ok(10));
}
#[test]
fn test_q09_err() {
    assert!(sum_strings(&["1", "x", "3"]).is_err());
}

// --- Q10 ---
#[test]
fn test_q10_ok() {
    assert_eq!(
        parse_csv_row("Alice,30,85"),
        Ok(Student {
            name: "Alice".to_string(),
            age: 30,
            score: 85
        })
    );
}
#[test]
fn test_q10_negative_age() {
    assert_eq!(parse_csv_row("Bob,-1,85"), Err(AppError::NegativeInput(-1)));
}
#[test]
fn test_q10_score_range() {
    assert_eq!(
        parse_csv_row("Carol,25,150"),
        Err(AppError::OutOfRange {
            value: 150,
            min: 0,
            max: 100
        })
    );
}
#[test]
fn test_q10_wrong_fields() {
    assert!(matches!(
        parse_csv_row("Alice,30"),
        Err(AppError::ParseError(_))
    ));
}
