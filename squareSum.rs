fn square_sum(n: isize) -> isize {
    // FizzBuzzと同じくレンジリテラル
    (0..n)
      // 高階関数の`filter`とクロージャリテラルの`|i| i % 2 == 0`
      .filter(|i| i % 2 == 0)
      // 同じく高階関数の`map`
      .map(|i| i * i)
      // イテレータへの演算`sum`
      .sum()
      // returnを書かなくても最後の値が返り値になる。
}

fn main() {
    println!("{}", square_sum(10));
}