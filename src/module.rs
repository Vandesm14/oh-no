use rune::{Any, ContextError, Module};

#[derive(Debug, Any)]
pub struct External {
  #[rune(get)]
  pub value: u32,
}

pub fn module() -> Result<Module, ContextError> {
  let mut module = Module::new();
  module.ty::<External>()?;
  Ok(module)
}
