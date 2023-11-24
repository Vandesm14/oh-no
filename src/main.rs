use std::{error::Error, sync::Arc};

use oh_no::*;
use rune::{
  termcolor::{ColorChoice, StandardStream},
  Context, Diagnostics, Source, Sources, Vm,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut world = World::default();

  let computer_a = world.add_computer(create_computer(0)?);
  let computer_b = world.add_computer(create_computer(1)?);
  let computer_c = world.add_computer(create_computer(2)?);

  world.connect(computer_a, computer_b);
  world.connect(computer_b, computer_c);

  world.update();

  Ok(())
}

fn create_computer(id: ComputerId) -> Result<Computer, Box<dyn Error>> {
  let m = module()?;

  let mut context = Context::with_default_modules()?;
  context.install(m)?;
  let runtime = Arc::new(context.runtime()?);

  let mut sources = Sources::new();
  sources.insert(Source::new(
    "rom",
    std::fs::read_to_string("roms/hello.rn")?,
  )?)?;

  let mut diagnostics = Diagnostics::new();

  let result = rune::prepare(&mut sources)
    .with_context(&context)
    .with_diagnostics(&mut diagnostics)
    .build();

  if !diagnostics.is_empty() {
    let mut writer = StandardStream::stderr(ColorChoice::Auto);
    diagnostics.emit(&mut writer, &sources)?;
  }

  let unit = result?;
  let vm = Vm::new(runtime, Arc::new(unit));

  Ok(Computer {
    vm,
    id,
    incoming: vec![],
  })
}
