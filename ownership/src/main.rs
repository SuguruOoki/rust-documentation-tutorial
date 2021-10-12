fn main() {
    let mut s = String::from("hello");
    println!("{}", s); // これは`hello, world!`と出力する

    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える

    println!("{}", s); // これは`hello, world!`と出力する
}
