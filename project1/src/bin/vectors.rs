fn main() {
  println!("Playground for vectors :D");

  let mut v: Vec<i32> = Vec::new();
  let v2 = vec![1,2,3];

  v.push(42);
  v.push(43);
  v.push(44);
  v.push(45);

  for i in &v {
    println!("{}", i);
  }

  // v[4]; // => will make the code panics ! :D

  match v.get(4) {
    Some(fifth) => println!("{}", fifth),
    None => println!("There is such element as the 'fifth'"),
  }

  match v.get(3) {
    Some(fourth) => println!("the fourth is : '{}'", fourth),
    None => println!("There is such element as the 'fourth'"),
  }
}