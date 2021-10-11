use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 1~100までの乱数を生成
    // gen_rangeメソッドは二つの数字を引数に取り、 それらの間の乱数を生成してくれます。
    // 範囲は下限値を含み、上限値を含まないため、1と101と指定しないと1から100の範囲の数字は得られません。
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);    //秘密の数字は次の通り: {}

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            // 行の読み込みに失敗しました
            .expect("Failed to read line");

        // 1. ユーザがエンターを押した
        // 2. 改行文字が追加される
        // 3. 改行文字をtrimする
        // 4. 文字列のparseメソッドは、文字列を解析して何らかの数値にする
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 次のように予想しました: {}
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),       //小さすぎ！
            Ordering::Greater => println!("Too big!"),      //大きすぎ！
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
