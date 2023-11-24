use rune::{Any, ContextError, Module};

#[derive(Debug, Any, Clone)]
pub struct External {
  #[rune(get, set)]
  pub id: u32,

  #[rune(get, set)]
  pub outgoing: Vec<u32>,
}

pub fn module() -> Result<Module, ContextError> {
  let mut module = Module::new();
  module.ty::<External>()?;
  Ok(module)
}
