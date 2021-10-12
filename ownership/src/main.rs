fn main() {
    // let mut s = String::from("hello");
    // println!("{}", s); // これは`hello, world!`と出力する
    // s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
    // println!("{}", s); // これは`hello, world!`と出力する

    // let s1 = String::from("hello");
    // let s2 = s1;

    // // println!("{}, world!", s1);
    // println!("{}, world!", s2);
    // // (注釈: ムーブが起きたのは、`s1`が`std::string::String`という
    // //     `Copy`トレイトを実装していない型だからです)

    // PHPの参照系のものと同じような扱いをすれば良いという理解。
    // オブジェクトクラスみたいなもん。
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("{}, {}, world!", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
