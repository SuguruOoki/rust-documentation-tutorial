fn main() {
    // 不変変数x
    //  let x = 5;
    // mutキーワードが使われると、xが束縛している値を5から6に変更可能
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // シャドーイング
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 定数にはmutキーワードは使用不可: 定数は標準で不変であるだけでなく、常に不変
    // 命名規則は、 全て大文字でアンダースコアで単語区切り
    const MAX_POINTS: u32 = 100_000;
    println!("The value of x is: {}", MAX_POINTS);

}