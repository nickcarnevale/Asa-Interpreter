extern crate nom;
extern crate asalang;
use std::env;
use std::fs;
use asalang::*;

fn main() -> Result<(), AsaErrorKind> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Usage: asac <filename>");
    return Ok(());
  }
  let filename = &args[1];
  let content = fs::read_to_string(filename).map_err(|e| AsaErrorKind::IoError(e.to_string()))?;
  let tokens = lex(&content);
  match program(tokens) {
    Ok((tokens, tree)) => {
      //println!("{:?}", tokens);
      //println!("Tree: {:#?}", tree);
      let mut interpreter = Interpreter::new();
      let result = interpreter.exec(&tree);

      let main_result = interpreter.start_main(vec![]);
      if let Ok(value) = main_result {
        println!("{:?}", value);
    } else {
        println!("An error occurred.");
    }
    },
    Err(e) => println!("Error: {:?}", e),
  }

  Ok(())
}
