#[allow(dead_code)]
pub fn run() {
  let keys = vec!["a", "b", "c"];
  // let keys : Vec<&stc> = Vew::new(); // push,remove
  for key in keys.iter() {
    println!("{}", key);
  }
  // 不用.iter()的话，遍历完keys之后，就获取不到keys了
  for (index, key) in keys.iter().enumerate() {
    println!("{}: {}", index, key);
  }

  let mut i: i8 = 1; // mut可变变量
  loop {
    println!("{}", i);
    i += 1;
    if i > 3 {
      break;
    }
  }
  for i in 1..3 {
    println!("{}", i);
  }
  for i in 1..=3 {
    println!("{}", i);
  }
}