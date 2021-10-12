fn main() {
    // 不変変数x
    //  let x = 5;
    // mutキーワードが使われると、xが束縛している値を5から6に変更可能
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // シャドーイング
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x);

    // 定数にはmutキーワードは使用不可: 定数は標準で不変であるだけでなく、常に不変
    // 命名規則は、 全て大文字でアンダースコアで単語区切り
    // const MAX_POINTS: u32 = 100_000;
    // println!("The value of x is: {}", MAX_POINTS);

    // スカラー型
    // 1. 整数
    // 2. 浮動小数点数
    // 3. 論理値
    // 4. 文字
    // let guess: u32 = "42".parse().expect("Not a number!");

    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32
    // println!("x: {}", x);
    // println!("y: {}", y);

    // Rustのchar型は、ユニコードのスカラー値を表します。
    // これはつまり、アスキーよりもずっとたくさんのものを表せるということ。
    // 絵文字もなんならちゃんと出力されちゃう。素敵。
    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻'; //ハート目の猫

    // println!("c: {}", c);
    // println!("z: {}", z);
    // println!("heart_eyed_cat: {}", heart_eyed_cat);

    // 複合型
    // タプル型
    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // // x[0]みたいなもん。
    // let five_hundred = x.0;

    // // x[1]みたいなもん。
    // let six_point_four = x.1;

    // // x[2]みたいなもん。
    // let one = x.2;

    // // println!("tup x: {}", x);
    // println!("five_hundred: {}", five_hundred);
    // println!("six_point_four: {}", six_point_four);
    // println!("one: {}", one);

    // 配列型
    // let a = [1, 2, 3, 4, 5];
    // let first = a[0];
    // let second = a[1];

    // println!("first: {}", first);
    // println!("second: {}", second);

    // 配列で無効なインデックスにアクセス
    // let a = [1, 2, 3, 4, 5];
    // let index = 10;

    // let element = a[index];

    // println!("The value of element is: {}", element); // 要素の値は{}です
}
