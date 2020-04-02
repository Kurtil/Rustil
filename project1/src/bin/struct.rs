fn main() {
  struct Test {
    number: i32,
    message: String
  }

  let test = Test {
    number: 42,
    message: String::from("jeanjean"),
  };

  let cloned_test = Test {
    ..test
  };

  println!("The cloned struct number is '{}' and messsage '{}'", cloned_test.number, cloned_test.message);

  // simple tuple struct

  struct Color(i32, i32, i32);

  fn do_color(r: i32, g: i32, b: i32) -> Color {
    Color(r,g,b)
  }

  let color = do_color(0,0,255);
}