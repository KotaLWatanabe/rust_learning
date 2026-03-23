// ============================================================
// Rust学習 フェーズ6: イテレータ・クロージャ
// ============================================================
// 実行方法:
//   cargo test --test phase6_iterators          # このファイル全体
//   cargo test --test phase6_iterators test_r01 # 復習だけ
//   cargo test --test phase6_iterators test_q01 # 新問Q1だけ
//
// Scalaとの対応:
//   .iter()         ≒ .iterator
//   .map()          ≒ .map()
//   .filter()       ≒ .filter()
//   .fold()         ≒ .foldLeft()
//   .collect()      ≒ .toList / .toVector
//   .flat_map()     ≒ .flatMap()
//   .zip()          ≒ .zip()
//   .take() / .skip() ≒ .take() / .drop()
//   .enumerate()    ≒ .zipWithIndex
//   Iterator トレイト ≒ Iterator[+A]
//
// ★ Rustのイテレータは「遅延評価」。
//   collect() や for ループなど「消費」するまで実行されない。
//   Scala の lazy val / LazyList に相当。
// ============================================================

use std::collections::{HashMap, HashSet};

// ============================================================
// 【フェーズ5 復習】
// ============================================================

// R01: Result + ? 演算子
// 【問題】"幅x高さ" 形式の文字列を受け取り、面積 (i32) を返してください。
// 例: parse_area("10x5") => Ok(50)
//     parse_area("10")   => Err(...)
//     parse_area("10xa") => Err(...)

fn parse_area(s: &str) -> Result<i32, String> {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return Err("parts should be formated.".into());
    }
    let size1 = parts[0].parse::<i32>().map_err(|e| e.to_string())?;
    let size2 = parts[1].parse::<i32>().map_err(|e| e.to_string())?;
    Ok(size1 * size2)
}

// R02: カスタムエラー
// 【問題】以下の関数を実装してください。
// 0以上の f64 の平方根を返す。負なら Err("negative input") を返す。

fn safe_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err("negative input".into())
    } else {
        Ok(x.sqrt())
    }
}

// R03: Iterator + Result (collect)
// 【問題】文字列スライスを受け取り、全て f64 にパースした合計を返す。
// 1つでも失敗したら Err を返す。
// ★ for ループでなく Iterator のメソッドチェーンで実装すること。

fn sum_floats(strings: &[&str]) -> Result<f64, String> {
    let floats = strings
        .iter()
        .map(|s| s.parse::<f64>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(floats.iter().sum())
}

// ============================================================
// 【フェーズ6】イテレータ・クロージャ
// ============================================================

// ============================================================
// Q01: 基本のイテレータメソッド
// ============================================================
// Rust の Vec に対して .iter() を呼ぶと Iterator が得られる。
// Scala の collection メソッドとほぼ同じ感覚で書ける。
//
//   v.iter()              参照のイテレータ (&T)
//   v.into_iter()         所有権を移動するイテレータ (T)
//   v.iter_mut()          可変参照のイテレータ (&mut T)
//
// 【問題】以下を全てメソッドチェーンで実装してください（for ループ禁止）。

// (1) 偶数だけ2倍にした Vec を返す
fn double_evens(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|n| *n % 2 == 0)
        .map(|n| n * 2)
        .collect()
}

// (2) 文字列スライスを全て大文字にした Vec<String> を返す
fn to_uppercase_all(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| s.to_uppercase()).collect()
}

// (3) i32 スライスの合計を返す (.sum() が使える)
fn sum_iter(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

// ============================================================
// Q02: fold / reduce
// ============================================================
// fold は Scala の foldLeft に相当。
// 初期値とクロージャを受け取り、順番に畳み込む。
//
//   v.iter().fold(初期値, |累積, 要素| 新累積)
//
// 【問題】fold を使って以下を実装してください（他のメソッド禁止）。

// (1) 積 (全要素の掛け算)
fn product(numbers: &[i32]) -> i32 {
    // 空スライスは 1 を返す (乗算の単位元)
    numbers.iter().fold(1, |acc, n| acc * n)
}

// (2) 文字列スライスを連結 (区切りなし)
fn concat_all(words: &[&str]) -> String {
    words.iter().fold("".into(), |mut acc, word| {
        acc.push_str(word);
        acc
    })
}

// (3) 最大値を fold で (max() メソッド禁止)
fn max_fold(numbers: &[i32]) -> Option<i32> {
    numbers.iter().fold(None, |max, &n| match max {
        Some(x) => {
            if x > n {
                Some(x)
            } else {
                Some(n)
            }
        }
        None => Some(n),
    })
}

// ============================================================
// Q03: flat_map
// ============================================================
// flat_map は Scala の flatMap と同じ。
// 各要素をイテレータに変換して、それをフラットにする。
//
// 【問題】以下を flat_map で実装してください。

// (1) Vec<Vec<i32>> をフラットにした Vec<i32> を返す
fn flatten_vecs(nested: Vec<Vec<i32>>) -> Vec<i32> {
    nested.into_iter().flat_map(|v| v.into_iter()).collect()
}

// (2) 文字列スライスを受け取り、各文字列を単語に分割した Vec<String> を返す
// 例: expand_words(&["hello world", "foo bar"]) => ["hello", "world", "foo", "bar"]
fn expand_words(sentences: &[&str]) -> Vec<String> {
    sentences
        .iter()
        .flat_map(|sentence| sentence.split_whitespace().map(|s| s.to_string()))
        .collect()
}

// ============================================================
// Q04: enumerate と zip
// ============================================================
// enumerate() ... (index, &value) のタプルを返す (Scala の zipWithIndex)
// zip()       ... 2つのイテレータを (a, b) タプルのイテレータに合わせる
//
// 【問題】

// (1) 文字列スライスを受け取り、"0: apple", "1: banana" 形式の Vec<String> を返す
fn indexed_list(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, item)| format!("{}: {}", i, item))
        .collect()
}

// (2) 2つのスライスを受け取り、対応する要素の積の Vec を返す
// 長さが短い方に合わせる (zip の挙動)
// 例: zip_multiply(&[1,2,3], &[4,5,6]) => [4, 10, 18]
fn zip_multiply(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().zip(b).map(|(&an, &bn)| an * bn).collect()
}

// ============================================================
// Q05: take / skip / chain
// ============================================================
// take(n)  ... 最初の n 要素だけ (Scala の take)
// skip(n)  ... 最初の n 要素を飛ばす (Scala の drop)
// chain()  ... 2つのイテレータを連結 (Scala の ++)
//
// 【問題】

// (1) スライスの先頭 n 個を除いた残りを Vec で返す
fn drop_first(numbers: &[i32], n: usize) -> Vec<i32> {
    numbers.iter().skip(n).copied().collect()
}

// (2) 2つのスライスを連結した後、偶数だけを返す
fn merge_evens(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter()
        .chain(b)
        .filter(|n| **n % 2 == 0)
        .copied()
        .collect()
}

// ============================================================
// Q06: クロージャのキャプチャ
// ============================================================
// クロージャは外側の変数を「キャプチャ」できる。
// Scala のラムダ式と同じだが、Rust では所有権ルールが適用される。
//
//   move |x| x + n   ... n の所有権をクロージャが奪う
//   |x| x + &n       ... n を借用する (n はまだ使える)
//
// 【問題】

// (1) 閾値を受け取り、「その値より大きいか判定するクロージャ」を返す
fn make_greater_than(threshold: i32) -> impl Fn(i32) -> bool {
    // ヒント: move |x| x > threshold
    move |x| x > threshold
}

// (2) prefix を受け取り、「その文字列を先頭に付けるクロージャ」を返す
fn make_prefixer(prefix: String) -> impl Fn(&str) -> String {
    // ヒント: move |s| format!("{}{}", prefix, s)
    move |s| format!("{}{}", prefix, s)
}

// ============================================================
// Q07: カスタム Iterator の実装
// ============================================================
// Iterator トレイトを実装すると、全てのイテレータメソッドが使える。
// 必要なのは next() メソッドだけ。
//
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
//     // map, filter, fold... は全てデフォルト実装
// }
//
// 【問題】フィボナッチ数列のイテレータを実装してください。
// FibIterator::new() で作成し、next() を呼ぶたびに次の値を返す。

struct FibIterator {
    a: u64,
    b: u64,
}

impl FibIterator {
    fn new() -> FibIterator {
        // 初期値: a=0, b=1
        Self { a: 0, b: 1 }
    }
}

impl Iterator for FibIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next = self.a;
        self.a = self.b;
        self.b = next + self.b;
        Some(next)
    }
}

// ============================================================
// Q08: Iterator アダプタの連鎖 (実践)
// ============================================================
// 複数のアダプタを連鎖させて複雑な処理を簡潔に書く。
// Scala の collection 操作と同じ感覚。
//
// 【問題】以下をメソッドチェーンで実装してください。

// (1) 数値スライスを受け取り、
//     正の数だけ選んで2乗し、100未満のものを昇順で返す
fn filtered_squares(numbers: &[i32]) -> Vec<i32> {
    let mut filtered: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n > 0)
        .map(|&n| n * n)
        .filter(|&n| n < 100)
        .collect();
    filtered.sort();
    filtered
}

// (2) 文字列スライスを受け取り、
//     長さ3以上の単語の最初の文字を大文字にして重複を除いた Vec<String> を返す
//     ※アルファベット順にソートして返す
fn unique_long_words(words: &[&str]) -> Vec<String> {
    let filtered: HashSet<String> = words
        .iter()
        .filter(|w| w.len() >= 3)
        .map(|w| capitalize(w))
        .collect();
    let mut sorted: Vec<String> = filtered.into_iter().collect();
    sorted.sort();
    sorted
}
fn capitalize(w: &str) -> String {
    let mut c = w.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

// ============================================================
// Q09: HashMap とイテレータ
// ============================================================
// HashMap もイテレータと組み合わせると強力。
//
// 【問題】

// (1) Vec<(String, i32)> を受け取り、HashMap<String, i32> に変換する
//     キーが重複する場合は大きい方の値を残す
fn vec_to_map(pairs: Vec<(String, i32)>) -> HashMap<String, i32> {
    pairs.into_iter().fold(HashMap::new(), |mut map, (k, v)| {
        let entry = map.entry(k).or_insert(v);
        if *entry < v {
            *entry = v;
        }
        map
    })
}

// (2) HashMap<String, Vec<i32>> を受け取り、
//     各キーの Vec の平均値の HashMap を返す
fn average_map(data: &HashMap<String, Vec<i32>>) -> HashMap<String, f64> {
    data.iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.iter().copied().sum::<i32>() as f64 / v.len() as f64,
            )
        })
        .collect()
}

// ============================================================
// Q10: 総合問題 - テキスト解析
// ============================================================
// 【問題】テキストを受け取り、以下の統計を返す関数を実装してください。
// 全て Iterator のメソッドチェーンを活用すること (for ループは最小限に)。

#[derive(Debug, PartialEq)]
struct TextStats {
    word_count: usize,    // 総単語数
    unique_words: usize,  // ユニーク単語数 (小文字化して重複除去)
    longest_word: String, // 最長単語 (同じ長さなら最初のもの)
    avg_word_length: f64, // 平均単語長
}

fn analyze_text(text: &str) -> TextStats {
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_set: HashSet<String> = words.iter().copied().map(|w| w.to_lowercase()).collect();
    let longest_word: String = words
        .iter()
        .max_by_key(|w| w.len())
        .unwrap_or(&"")
        .to_string();
    let avg_word_length = words.iter().map(|w| w.len() as f64).sum::<f64>() / (words.len() as f64);
    TextStats {
        word_count: words.len(),
        unique_words: word_set.len(),
        longest_word,
        avg_word_length,
    }
}

// ============================================================
// テスト (変更不要)
// ============================================================

// --- 復習 ---
#[test]
fn test_r01_area_ok() {
    assert_eq!(parse_area("10x5"), Ok(50));
}
#[test]
fn test_r01_area_err() {
    assert!(parse_area("10").is_err());
    assert!(parse_area("10xa").is_err());
}

#[test]
fn test_r02_sqrt_ok() {
    assert!((safe_sqrt(4.0).unwrap() - 2.0).abs() < 1e-9);
}
#[test]
fn test_r02_sqrt_err() {
    assert!(safe_sqrt(-1.0).is_err());
}

#[test]
fn test_r03_sum_floats_ok() {
    assert!((sum_floats(&["1.5", "2.5", "3.0"]).unwrap() - 7.0).abs() < 1e-9);
}
#[test]
fn test_r03_sum_floats_err() {
    assert!(sum_floats(&["1.0", "abc"]).is_err());
}

// --- Q01 ---
#[test]
fn test_q01_double_evens() {
    assert_eq!(double_evens(&[1, 2, 3, 4, 5, 6]), vec![4, 8, 12]);
}
#[test]
fn test_q01_uppercase() {
    assert_eq!(
        to_uppercase_all(&["hello", "world"]),
        vec!["HELLO", "WORLD"]
    );
}
#[test]
fn test_q01_sum() {
    assert_eq!(sum_iter(&[1, 2, 3, 4, 5]), 15);
}

// --- Q02 ---
#[test]
fn test_q02_product() {
    assert_eq!(product(&[1, 2, 3, 4]), 24);
    assert_eq!(product(&[]), 1);
}
#[test]
fn test_q02_concat() {
    assert_eq!(concat_all(&["foo", "bar", "baz"]), "foobarbaz");
}
#[test]
fn test_q02_max_fold() {
    assert_eq!(max_fold(&[3, 1, 4, 1, 5, 9]), Some(9));
    assert_eq!(max_fold(&[]), None);
}

// --- Q03 ---
#[test]
fn test_q03_flatten() {
    assert_eq!(
        flatten_vecs(vec![vec![1, 2], vec![3, 4], vec![5]]),
        vec![1, 2, 3, 4, 5]
    );
}
#[test]
fn test_q03_expand_words() {
    assert_eq!(
        expand_words(&["hello world", "foo bar"]),
        vec!["hello", "world", "foo", "bar"]
    );
}

// --- Q04 ---
#[test]
fn test_q04_indexed() {
    assert_eq!(
        indexed_list(&["apple", "banana"]),
        vec!["0: apple", "1: banana"]
    );
}
#[test]
fn test_q04_zip_multiply() {
    assert_eq!(zip_multiply(&[1, 2, 3], &[4, 5, 6]), vec![4, 10, 18]);
    assert_eq!(zip_multiply(&[1, 2, 3], &[4, 5]), vec![4, 10]); // 短い方に合わせる
}

// --- Q05 ---
#[test]
fn test_q05_drop_first() {
    assert_eq!(drop_first(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5]);
    assert_eq!(drop_first(&[1, 2], 5), vec![]);
}
#[test]
fn test_q05_merge_evens() {
    assert_eq!(merge_evens(&[1, 2, 3], &[4, 5, 6]), vec![2, 4, 6]);
}

// --- Q06 ---
#[test]
fn test_q06_greater_than() {
    let gt5 = make_greater_than(5);
    assert!(gt5(6));
    assert!(!gt5(5));
    assert!(!gt5(3));
}
#[test]
fn test_q06_prefixer() {
    let add_hello = make_prefixer("Hello, ".to_string());
    assert_eq!(add_hello("world"), "Hello, world");
    assert_eq!(add_hello("Rust"), "Hello, Rust");
}

// --- Q07 ---
#[test]
fn test_q07_fib_first_8() {
    let fibs: Vec<u64> = FibIterator::new().take(8).collect();
    assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13]);
}
#[test]
fn test_q07_fib_sum() {
    // フィボナッチ数列の最初の10項の合計
    let sum: u64 = FibIterator::new().take(10).sum();
    assert_eq!(sum, 88);
}

// --- Q08 ---
#[test]
fn test_q08_filtered_squares() {
    let result = filtered_squares(&[-3, -1, 0, 2, 5, 10, 11]);
    assert_eq!(result, vec![4, 25]); // 2²=4, 5²=25 (10²=100, 11²=121 は除外)
}
#[test]
fn test_q08_unique_long_words() {
    let words = vec!["the", "quick", "brown", "fox", "the", "quick"];
    let result = unique_long_words(&words);
    // 長さ3以上: "the"(3), "quick"(5), "brown"(5), "fox"(3)
    // 先頭大文字 + 重複除去 + アルファベット順
    assert_eq!(result, vec!["Brown", "Fox", "Quick", "The"]);
}

// --- Q09 ---
#[test]
fn test_q09_vec_to_map() {
    let pairs = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 3),
        ("a".to_string(), 2), // "a" は大きい方 2 を残す
    ];
    let map = vec_to_map(pairs);
    assert_eq!(map["a"], 2);
    assert_eq!(map["b"], 3);
}
#[test]
fn test_q09_average_map() {
    let mut data = HashMap::new();
    data.insert("math".to_string(), vec![80, 90, 100]);
    data.insert("english".to_string(), vec![70, 80]);
    let result = average_map(&data);
    assert!((result["math"] - 90.0).abs() < 1e-9);
    assert!((result["english"] - 75.0).abs() < 1e-9);
}

// --- Q10 ---
#[test]
fn test_q10_analyze() {
    let stats = analyze_text("the quick brown fox jumps over the lazy dog");
    assert_eq!(stats.word_count, 9);
    assert_eq!(stats.unique_words, 8); // "the" が2回
    assert_eq!(stats.longest_word, "quick"); // 5文字 (quick/brown/jumps は同じ長さ → 最初)
    assert!((stats.avg_word_length - (35.0 / 9.0)).abs() < 0.01);
}
#[test]
fn test_q10_single_word() {
    let stats = analyze_text("hello");
    assert_eq!(stats.word_count, 1);
    assert_eq!(stats.unique_words, 1);
    assert_eq!(stats.longest_word, "hello");
    assert!((stats.avg_word_length - 5.0).abs() < 1e-9);
}
