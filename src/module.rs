use rune::{Any, ContextError, Module};

#[derive(Debug, Any, Copy, Clone)]
pub struct External {
  #[rune(get)]
  pub value: u32,
}

#[rune::function(instance)]
fn print(val: u32) {
  println!("printing from Rust: {}", val);
}

pub fn module() -> Result<Module, ContextError> {
  let mut module = Module::with_item(["mymodule"])?;
  module.ty::<External>()?;
  module.function_meta(print)?;

  Ok(module)
}
