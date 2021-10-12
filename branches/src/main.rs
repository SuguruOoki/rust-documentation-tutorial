fn main() {
    // let number = 3;

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
    // if number != 0 {
    //     println!("number was something other than zero"); // 数値は0以外の何かです
    // }

    // let 文内で　　if 式を使う
    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // // numberの値は、{}です
    // println!("The value of number is: {}", number);

    // loop {
    //     println!("again!"); // また
    // }

    let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < a.len() {
    //     // 値は{}です
    //     println!("the value is: {}", a[index]);

    //     index = index + 1;
    // }

    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }
}
