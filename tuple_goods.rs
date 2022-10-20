fn main() {
  // タプルを作る
  let banana = ("バナナ", 300);
  let apple = ("りんご", 200);
  // タプルを参照して合計金額を求める
  let total = banana.1 + apple.1;
  // タプルの内容を表示
  print_tuple(&banana);
  print_tuple(&apple);
  println!("合計{}円です", total);
}
// (商品名, 金額)のタプルを表示する関数
fn print_tuple(item: &(&str, i64)) {
  println!("{}を{}円で購入", item.0, item.1);
}

