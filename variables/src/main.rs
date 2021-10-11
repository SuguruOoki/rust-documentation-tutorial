fn main() {
    // 不変変数x
    //  let x = 5;
    // mutキーワードが使われると、xが束縛している値を5から6に変更可能
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}