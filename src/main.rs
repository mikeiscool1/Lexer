use std::env;
use std::fs;
use std::path::Path;
use std::process;

mod lexer;
mod highlight;

use lexer::Lexer;

fn main() {
  let args = env::args().collect::<Vec<String>>();

  if args.len() < 2 {
    println!("Shell not implemented.");
    process::exit(1);
  }

  let file_name = &args[1];
  let path = Path::new(file_name.as_str());

  let absolute_path = fs::canonicalize(path);
  if absolute_path.is_err() {
    println!("Error: No such file or directory.");
    process::exit(1);
  }
  
  let file_content = fs::read_to_string(absolute_path.as_ref().unwrap());
  if file_content.is_err() {
    println!("Error: Could not read the file.");
    process::exit(1);
  }

  let mut lex = Lexer::new();
  let result = lex.analyze(&file_content.unwrap());

  if result.is_err() {
    println!("Syntax error at {}:{}:{}: {}", absolute_path.as_ref().unwrap().to_str().unwrap(), lex.token_start.row + 1, lex.token_start.col + 1, result.err().unwrap());
    process::exit(1);
  }

  highlight::print_tokens(&lex.tokens);

  //println!("{:?}", lex.tokens);
}
