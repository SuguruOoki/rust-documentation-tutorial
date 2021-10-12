## 概要

https://doc.rust-jp.rs/book-ja/title-page.html

上記を進めるために用意したリポジトリです。


## バージョン

開始当初

```shell
~/workspace/rust-projects > cargo --version
cargo 1.55.0 (32da73ab1 2021-08-23)
```

## メモ

### 数値まわり

| 大きさ | 符号付き | 符号なし |
| :----: | :------- | :------- |
| 8-bit  | i8       | u8       |
| 16-bit | i16      | u16      |
| 32-bit | i32      | u32      |
| 64-bit | i64      | u64      |
|  arch  | isize    | usize    |

1. 整数リテラルは何進数でも記述可能。
2. 浮動小数点数は、IEEE-754規格に従って表現

### 文字まわり

Rustのchar型は、ユニコードのスカラー値を表します。これはつまり、アスキーよりもずっとたくさんのものを表せるということ。

```rust
suguruohki@SugurunoMacBook-Pro-2:~/workspace/rust-projects/variables > cargo run                                                                                                            [main] ttys017 [10/12 23:09:31]
   Compiling variables v0.1.0 (/Users/suguruohki/workspace/rust-projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/variables`
c: z
z: ℤ
heart_eyed_cat: 😻
```

### 複合型

1. タプル
2. 配列

## エラー

```rust
help: if this is intentional, prefix it with an underscore: `_y`
```

ライフサイクル内で利用されていない変数があった際に提案される利用方法のエラー。
変数が利用されることで、解決される。
[Issue](https://github.com/rust-lang/rust/issues/66636) で記述されているとおり、このエラー文自体が不親切なのでは？と言われてはいる。

```rust
index out of bounds: the length is 5 but the index is 10
```

配列の無効なインデックスにアクセスした際に出るエラー。
配列の有効なインデックスのみにアクセスするようにロジックを変更すること

```rust
let x = (let y = 6);
```

let y = 6という文は値を返さないので、xに束縛するものがないわけです。これは、 CやRubyなどの言語とは異なる動作です。CやRubyでは、代入は代入値を返します。これらの言語では、 x = y = 6と書いて、xもyも値6になるようにできるのですが、Rustにおいては、 そうは問屋が卸さないわけです。

```rust
 expected `bool`, found integer
```

型が異なるということを示す。

```rust
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:10:28
   |
7  |     let s1 = String::from("hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
8  |     let s2 = s1;
   |              -- value moved here
9  |
10 |     println!("{}, world!", s1);
   |                            ^^ value borrowed here after move

```

すでにコピー後の箇所が利用されているということを示しており、s2を利用するべきということ。