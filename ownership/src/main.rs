// fn main() {
//     // let mut s = String::from("hello");
//     // println!("{}", s); // これは`hello, world!`と出力する
//     // s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える
//     // println!("{}", s); // これは`hello, world!`と出力する

//     // let s1 = String::from("hello");
//     // let s2 = s1;

//     // // println!("{}, world!", s1);
//     // println!("{}, world!", s2);
//     // // (注釈: ムーブが起きたのは、`s1`が`std::string::String`という
//     // //     `Copy`トレイトを実装していない型だからです)

//     // PHPの参照系のものと同じような扱いをすれば良いという理解。
//     // オブジェクトクラスみたいなもん。
//     // let s1 = String::from("hello");
//     // let s2 = s1.clone();
//     // println!("{}, {}, world!", s1, s2);

//     // let x = 5;
//     // let y = x;

//     // println!("x = {}, y = {}", x, y);
//     // TODO: [所有権と関数]から始める
// }

/**
 * ↓リスト4-3: 所有権とスコープが注釈された関数群
 */

fn main() {
    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ...
                        // ... ここではもう有効ではない

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
                   // 大丈夫
} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。
  //

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  //

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
