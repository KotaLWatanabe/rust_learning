// ============================================================
// Rust学習 フェーズ2: 所有権・借用・ライフタイム
// ============================================================
// 実行方法:
//   cargo test --test phase2_ownership          # このファイル全体
//   cargo test --test phase2_ownership test_r01 # 復習Q1だけ
//   cargo test --test phase2_ownership test_q01 # 新問Q1だけ
//
// ★ Scalaにない概念のため、各問題に詳しい解説をつけています。
//    コンパイルエラーが出たら、エラーメッセージをよく読んでください。
//    Rustのエラーメッセージは非常に丁寧で、解決策まで教えてくれます。
// ============================================================

// ============================================================
// 【フェーズ1 復習】
// ============================================================

// R01: for ループ + Vec
// 【問題】Vec<i32> を受け取り、各要素を2乗したVecを返してください。
// 例: square_all(vec![1, 2, 3, 4]) => vec![1, 4, 9, 16]

fn square_all(numbers: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for n in numbers {
        result.push(n * n);
    }
    result
}

// R02: match + Option
// 【問題】Option<i32> を受け取り、
//   Some(n) なら n * 2 を Some で包んで返す
//   None なら None を返す
// 関数を実装してください。

fn double_option(opt: Option<i32>) -> Option<i32> {
    // opt.map(|n| n * 2)
    match opt {
        Some(n) => Some(n * 2),
        None => None,
    }
}

// R03: String操作
// 【問題】文字列のVecを受け取り、カンマ区切りで結合したStringを返してください。
// 例: join_strings(vec!["a", "b", "c"]) => "a,b,c"
// ヒント: .join(",") メソッドが使えます。

fn join_strings(words: Vec<&str>) -> String {
    words.join(",")
}

// ============================================================
// 【フェーズ2】所有権・借用・ライフタイム
// ============================================================
//
// ★ Rustの最重要概念: 所有権 (Ownership) の3原則
//
//   1. 値は必ず1つの変数（オーナー）に所有される
//   2. オーナーがスコープを抜けると値は自動的に解放される (drop)
//   3. 所有権は移動 (move) するか、借用 (borrow) される
//
// Scalaとの最大の違い: GCが存在しない。代わりにコンパイル時に
// 所有権ルールでメモリ安全性を保証する。
// ============================================================

// ============================================================
// Q01: 所有権の移動 (Move)
// ============================================================
// Rustでは代入や関数呼び出しで所有権が「移動」する。
// 移動後の変数は使えなくなる (Scalaにはない概念)。
//
// 例:
//   let s1 = String::from("hello");
//   let s2 = s1;  // s1の所有権がs2に移動
//   // s1はもう使えない! コンパイルエラーになる
//
// 【問題】文字列を受け取り、元の文字列に " world" を追加したものを返してください。
// s を受け取ったら所有権はこの関数にある。自由に変更してOK。
// ヒント: push_str() で文字列を追加できる

fn append_world(mut s: String) -> String {
    s.push_str(" world");
    s
}

// ============================================================
// Q02: Clone (複製)
// ============================================================
// 所有権を移動せずコピーしたい場合は .clone() を使う。
// ただし clone はメモリ確保を伴うので、不必要に使わないのが Rust スタイル。
// (ScalaのimmutableなListは内部的に共有されるが、Rustでは明示的にcloneが必要)
//
// 【問題】2つの文字列を受け取り、両方を連結した新しいStringを返してください。
// s1, s2 の所有権は呼び出し元に残したい。
// → 引数を &String (参照) にするか、内部でcloneするか選んでください。
// ヒント: この問題は &str を引数にするのが最もRustらしい

fn concat_strings(s1: &str, s2: &str) -> String {
    let mut s = s1.to_string();
    s.push_str(s2);
    s
}

// ============================================================
// Q03: 不変借用 (Immutable Borrow) &T
// ============================================================
// 所有権を移動せず「参照」として渡すのが借用。
//
//   &T  ... 不変参照 (読み取り専用)
//           同時に複数持てる
//
// Scalaで val の参照を渡すようなイメージ。
//
// 【問題】整数のスライスを受け取り、合計を返してください。
// 所有権を奪わず参照で受け取ること (&[i32])。

fn sum_slice(numbers: &[i32]) -> i32 {
    //    numbers.iter().sum()
    let mut sum = 0;
    for &n in numbers {
        sum += n;
    }
    sum
}

// ============================================================
// Q04: 不変借用 - 構造体への参照
// ============================================================
// 【問題】2つの文字列スライスのうち、長い方を返してください。
// どちらも参照のまま返す (所有権を移動しない)。
// 戻り値も &str (参照) であることに注意。

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // ★ 'a はライフタイム注釈。「s1とs2が生きている間、戻り値も有効」という意味。
    //    ここでは実装だけ考えてOK。ライフタイムの詳細はQ09で扱います。
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// ============================================================
// Q05: 可変借用 (Mutable Borrow) &mut T
// ============================================================
// &mut T ... 可変参照 (読み書き可能)
//            同時に1つしか持てない (データ競合を防ぐ)
//
// ★ 借用の黄金ルール:
//   ・不変参照 (&T)  は同時にいくつでもOK
//   ・可変参照 (&mut T) は同時に1つだけ
//   ・不変と可変は同時に持てない
//
// 【問題】Vec<i32> への可変参照を受け取り、全要素を2倍にしてください。
// 戻り値はなし (参照を通じて直接変更する)。

fn double_all(numbers: &mut Vec<i32>) {
    for n in numbers {
        *n *= 2;
    }
}

// ============================================================
// Q06: スライス参照 &[T]
// ============================================================
// &[T] は Vec<T> や配列 [T; N] への参照 (スライス)。
// 関数の引数では Vec<T> より &[T] の方が柔軟でRustらしい。
//
// 【問題】i32のスライスを受け取り、最初の要素と最後の要素のタプルを返してください。
// スライスが空なら None を返すこと。

fn first_and_last(slice: &[i32]) -> Option<(i32, i32)> {
    // ヒント: slice.first(), slice.last() が使えます。
    //         またはインデックス slice[0], slice[slice.len()-1]
    match (slice.first(), slice.last()) {
        (Some(f), Some(l)) => Some((*f, *l)),
        _ => None,
    }
}

// ============================================================
// Q07: 文字列スライス &str
// ============================================================
// &str は String のスライス (文字列への参照)。
// String::from("hello") で作った String に対して
// &s や &s[..] で &str を取得できる。
//
// 【問題】文字列を受け取り、最初の単語を返してください。
// スペースが見つかれば、そこまでのスライスを返す。
// スペースがなければ文字列全体を返す。
// 例: first_word("hello world") => "hello"
//     first_word("hello")       => "hello"

fn first_word(s: &str) -> &str {
    // ヒント: s.find(' ') でスペースの位置を Option<usize> で取得できる
    //         &s[..index] でスライスを取得できる
    s.find(' ').map_or(s, |i| &s[..i])
}

// ============================================================
// Q08: 所有権を返す (所有権の返却パターン)
// ============================================================
// 関数に所有権を渡して、処理後に返してもらうパターン。
// 借用が使えない場合や、変換して返す場合に使う。
//
// 【問題】文字列Vecを受け取り、
//   - 空文字列を除いた要素のみを含む新しいVecを返してください。
// 例: filter_empty(vec!["a".to_string(), "".to_string(), "b".to_string()])
//     => vec!["a", "b"]

fn filter_empty(strings: Vec<String>) -> Vec<String> {
    // ヒント: for s in strings { if !s.is_empty() { result.push(s); } }
    //         所有権がループ変数 s に移動していることに注目
    let mut result = Vec::new();
    for s in strings {
        if !s.is_empty() {
            result.push(s);
        }
    }
    result
}

// ============================================================
// Q09: ライフタイム注釈 'a
// ============================================================
// ライフタイムとは「参照が有効な期間」のこと。
// 通常はコンパイラが推論するが、複数の参照が絡む場合は明示が必要。
//
// ★ ライフタイムの読み方:
//   fn foo<'a>(x: &'a str, y: &'a str) -> &'a str
//   「x と y が両方生きている間、戻り値の参照も有効」
//
// 【問題】文字列スライスと区切り文字を受け取り、
// 区切り文字より前の部分を返してください。
// 区切り文字が見つからない場合は文字列全体を返す。
// 例: before_delimiter("hello:world", ':') => "hello"
//     before_delimiter("hello", ':')       => "hello"

fn before_delimiter<'a>(s: &'a str, delimiter: char) -> &'a str {
    s.find(delimiter).map_or(s, |i| &s[..i])
}

// ============================================================
// Q10: 総合問題 - 単語カウンター
// ============================================================
// 【問題】テキストを受け取り、各単語の出現回数を数えて返してください。
// 戻り値は Vec<(String, usize)> で、出現回数の多い順に並べること。
// 例: word_count("the cat sat on the mat")
//     => vec![("the", 2), ("cat", 1), ("mat", 1), ("on", 1), ("sat", 1)]
//        (theが2回で先頭、残りは出現1回でアルファベット順)
//
// ヒント:
//   - text.split_whitespace() で単語に分割
//   - std::collections::HashMap を使って集計
//   - .sort_by() でソート

use std::collections::HashMap;

fn word_count(text: &str) -> Vec<(String, usize)> {
    let words = text.split_whitespace();
    let mut map: HashMap<String, usize> = HashMap::new();
    for word in words {
        map.entry(word.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut entries = Vec::from_iter(map);
    entries.sort_by(|(w1, c1), (w2, c2)| c2.cmp(c1).then_with(|| w1.cmp(w2)));
    entries
}

// ============================================================
// テスト (変更不要)
// ============================================================

// --- 復習 ---
#[test]
fn test_r01_squares() {
    assert_eq!(square_all(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
}
#[test]
fn test_r01_empty() {
    assert_eq!(square_all(vec![]), vec![]);
}

#[test]
fn test_r02_some() {
    assert_eq!(double_option(Some(5)), Some(10));
}
#[test]
fn test_r02_none() {
    assert_eq!(double_option(None), None);
}

#[test]
fn test_r03_basic() {
    assert_eq!(join_strings(vec!["a", "b", "c"]), "a,b,c");
}
#[test]
fn test_r03_single() {
    assert_eq!(join_strings(vec!["only"]), "only");
}

// --- Q01 ---
#[test]
fn test_q01_append() {
    assert_eq!(append_world(String::from("hello")), "hello world");
}
#[test]
fn test_q01_empty() {
    assert_eq!(append_world(String::from("")), " world");
}

// --- Q02 ---
#[test]
fn test_q02_concat() {
    assert_eq!(concat_strings("hello", " world"), "hello world");
}
#[test]
fn test_q02_empty() {
    assert_eq!(concat_strings("", "abc"), "abc");
}

// --- Q03 ---
#[test]
fn test_q03_sum() {
    assert_eq!(sum_slice(&[1, 2, 3, 4, 5]), 15);
}
#[test]
fn test_q03_empty() {
    assert_eq!(sum_slice(&[]), 0);
}
#[test]
fn test_q03_negative() {
    assert_eq!(sum_slice(&[-1, -2, 3]), 0);
}

// --- Q04 ---
#[test]
fn test_q04_longer_first() {
    assert_eq!(longer("long string", "xyz"), "long string");
}
#[test]
fn test_q04_longer_second() {
    assert_eq!(longer("hi", "hello"), "hello");
}
#[test]
fn test_q04_equal() {
    // 同じ長さなら s1 か s2 どちらでもOK (テストは両方受け入れる)
    let result = longer("abc", "xyz");
    assert_eq!(result.len(), 3);
}

// --- Q05 ---
#[test]
fn test_q05_double() {
    let mut v = vec![1, 2, 3];
    double_all(&mut v);
    assert_eq!(v, vec![2, 4, 6]);
}
#[test]
fn test_q05_empty() {
    let mut v: Vec<i32> = vec![];
    double_all(&mut v);
    assert_eq!(v, vec![]);
}

// --- Q06 ---
#[test]
fn test_q06_basic() {
    assert_eq!(first_and_last(&[1, 2, 3, 4, 5]), Some((1, 5)));
}
#[test]
fn test_q06_single() {
    assert_eq!(first_and_last(&[42]), Some((42, 42)));
}
#[test]
fn test_q06_empty() {
    assert_eq!(first_and_last(&[]), None);
}

// --- Q07 ---
#[test]
fn test_q07_with_space() {
    assert_eq!(first_word("hello world"), "hello");
}
#[test]
fn test_q07_no_space() {
    assert_eq!(first_word("hello"), "hello");
}
#[test]
fn test_q07_multiple_spaces() {
    assert_eq!(first_word("one two three"), "one");
}

// --- Q08 ---
#[test]
fn test_q08_filter() {
    let input = vec![
        String::from("hello"),
        String::from(""),
        String::from("world"),
        String::from(""),
    ];
    assert_eq!(filter_empty(input), vec!["hello", "world"]);
}
#[test]
fn test_q08_all_empty() {
    let input = vec![String::from(""), String::from("")];
    assert_eq!(filter_empty(input), Vec::<String>::new());
}

// --- Q09 ---
#[test]
fn test_q09_with_delimiter() {
    assert_eq!(before_delimiter("hello:world", ':'), "hello");
}
#[test]
fn test_q09_no_delimiter() {
    assert_eq!(before_delimiter("hello", ':'), "hello");
}
#[test]
fn test_q09_at_start() {
    assert_eq!(before_delimiter(":world", ':'), "");
}

// --- Q10 ---
#[test]
fn test_q10_basic() {
    let result = word_count("the cat sat on the mat");
    // "the" が2回で先頭
    assert_eq!(result[0], ("the".to_string(), 2));
    // 残りは全て1回
    assert_eq!(result.len(), 5);
    result[1..]
        .iter()
        .for_each(|(_, count)| assert_eq!(*count, 1));
}
#[test]
fn test_q10_single_word() {
    let result = word_count("hello");
    assert_eq!(result, vec![("hello".to_string(), 1)]);
}
#[test]
fn test_q10_empty() {
    let result = word_count("");
    assert_eq!(result, vec![]);
}
