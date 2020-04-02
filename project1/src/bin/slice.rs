// slice does not have ownership

fn main() {
  let str = String::new("Hello world!");

  let hello = &str[..5];
  let world = &str[6..11];
}