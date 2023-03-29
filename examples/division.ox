func divide_two(i32 a, i32 b) ~ i32! {
  if b == 0 {
    panic("Tried to divide by 0!");
  }

  a / b
}
