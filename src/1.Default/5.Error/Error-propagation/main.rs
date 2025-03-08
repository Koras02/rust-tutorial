fn may_fail() -> Result<(), String> {
    // 실패할 수 있는 작업
    Err(String::from("Fail!"))
}

fn process() -> Result<(), String> {
      may_fail()?;
      Ok(())
}

fn main() {
      match process() {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: {}", e)
      }
}