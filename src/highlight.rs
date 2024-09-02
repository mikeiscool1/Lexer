use std::collections::HashMap;

use crate::lexer::{Location, Symbol, Token, ValueVariant, KEYWORDS, STR_ESCAPE_CODES, SYMBOLS, TYPES};

const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const PURPLE: &str = "\x1b[35m";
const RESET: &str = "\x1b[0m";

fn get_key_from_value<'a, K: Clone, V: PartialEq>(map: &'a HashMap<K, V>, value: &V) -> Option<&'a K> {
  for (key, val) in map.iter() {
    if *val == *value {
      return Some(key);
    }
  }

  None
}

pub fn print_tokens(tokens: &Vec<Token>) {
  let mut last_end = Location {
    row: 0,
    col: 0
  };

  for (i, token) in tokens.iter().enumerate() {
    let row_difference = token.start.row - last_end.row;
    let col_difference = if row_difference == 0 { token.start.col - last_end.col } else { token.start.col };

    for _ in 0..row_difference { println!() }
    for _ in 0..col_difference - 1 { print!(" ") }

    last_end = token.end;

    match &token.value {
      ValueVariant::Symbol(s) => {
        let key = *get_key_from_value(&SYMBOLS, &s).expect("Could not find symbol.");
        print!("{}", key);
      },
      ValueVariant::Keyword(k) => {
        let key = *get_key_from_value(&KEYWORDS, &k).expect("Could not find keyword.");
        print!("{}{}{}", PURPLE, key, RESET);
      },
      ValueVariant::Type(t) => {
        let key = *get_key_from_value(&TYPES, &t).expect("Could not find type.");
        print!("{}{}{}", PURPLE, key, RESET);
      },
      ValueVariant::Identifier(id) => {
        let mut color = RED;

        if i + 1 < tokens.len() {
          if let ValueVariant::Symbol(s) = tokens[i + 1].value {
            if s == Symbol::OpenParenthesis { color = BLUE; }
          }
        }

        print!("{}{}{}", color, id, RESET);
      },
      ValueVariant::Integer(int) => {
        print!("{}{}{}", YELLOW, int, RESET);
      },
      ValueVariant::Decimal(d) => {
        print!("{}{}{}", YELLOW, d, RESET);
      },
      ValueVariant::Boolean(b) => {
        print!("{}{}{}", YELLOW, b, RESET);
      },
      ValueVariant::String(s) => {
        let mut new_str = String::new();

        for c in s.chars() {
          if let Some(escape_code) = get_key_from_value(&STR_ESCAPE_CODES, &c) {
            new_str.push_str(CYAN);
            new_str.push('\\');
            new_str.push(*escape_code);
            new_str.push_str(GREEN);
          } else { new_str.push(c); }
        }

        print!("{}\"{}\"{}", GREEN, new_str, RESET);
      },
      ValueVariant::Character(c) => {
        if let Some(escape_code) = get_key_from_value(&STR_ESCAPE_CODES, &c) {
          print!("{}'{}\\{}{}'{}", GREEN, CYAN, escape_code, GREEN, RESET);
        } else {
          print!("{}{}{}", GREEN, c, RESET);
        }
      }
    }
  }
}