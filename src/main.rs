use std::sync::Arc;

use oh_no::*;
use rune::{
  termcolor::{ColorChoice, StandardStream},
  Context, Diagnostics, Source, Sources, Vm,
};

#[pollster::main]
async fn main() {
  let mut world = World::default();

  let context = Context::with_default_modules().unwrap();
  let runtime = Arc::new(context.runtime());

  let mut sources = Sources::new();
  sources.insert(Source::new(
    "rom",
    std::fs::read_to_string("roms/hello.rn").unwrap(),
  ));

  let mut diagnostics = Diagnostics::new();

  let result = rune::prepare(&mut sources)
    .with_context(&context)
    .with_diagnostics(&mut diagnostics)
    .build();

  if !diagnostics.is_empty() {
    let mut writer = StandardStream::stderr(ColorChoice::Auto);
    diagnostics.emit(&mut writer, &sources).unwrap();
  }

  let unit = result.unwrap();
  let vm = Vm::new(runtime.clone(), Arc::new(unit));

  let computer_a = world.add_computer(Computer { vm: vm.clone() });
  let computer_b = world.add_computer(Computer { vm: vm.clone() });
  let computer_c = world.add_computer(Computer { vm: vm.clone() });

  world.connect(computer_a, computer_b);
  world.connect(computer_b, computer_c);

  world.update();
}
