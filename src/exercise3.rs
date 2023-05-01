pub fn reverse(input: &str) -> String {
  let mut chars: Vec<char> = input.chars().collect();
 let len = chars.len();
  for i in 0..(len / 2) {
      chars.swap(i, len - i - 1);
  }
  return chars.into_iter().collect();
}


pub fn reverse(data: &str) -> String {
  data.chars().rev().collect()
}