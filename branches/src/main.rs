fn main() {
    let number = 3;

    // if number < 5 {
    //     println!("condition was true"); // 条件は真でした
    // } else {
    //     println!("condition was false"); // 条件は偽でした
    // }

    // この形式はPHPではtrueとなるが、Rustでは型が異なるためビルドエラーとなる。
    // if number {
    //     println!("number was three"); // 数値は3です
    // }

    // 比較されているため、この形式は問題なし。
    if number != 0 {
        println!("number was something other than zero"); // 数値は0以外の何かです
    }
}
