fn main() {
  let first = 10;
  let second = 8;

  let both = (first, second);

  println!("the added result is : '{}'", add(both));
}

fn add(both: (i32, i32)) -> i32 {
  both.0 + both.1
}