use std::io;

fn main() {
  println!("Type something to get it backwards:");

  let input = read_input();

  println!("Your string backwards is: {}", input.chars().rev().collect::<String>());
}

fn read_input() -> String {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).expect("Error while reading");
  buffer
}
