use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
  Plus, Minus, Mul, Div, Mod, PlusEquals, MinusEquals, MulEquals, DivEquals, ModEquals,
  Increment, Decrement, Assign, Equals, NotEquals, GreaterThan, LessThan, GreaterThanEqual, LessThanEqual,
  OpenParenthesis, CloseParenthesis, OpenBracket, CloseBracket, OpenBrace, CloseBrace,
  Not, And, Or, BitNot, BitAnd, BitOr, BitXor, BitLeftShift, BitRightShift,
  BitNotEquals, BitAndEquals, BitXorEquals, BitLeftShiftEquals, BitRightShiftEquals,
  Dot, Variadic, Comma, Question, Colon, Semicolon, Backslash, Hashtag, At, NamespaceAccess,
  Dollar, Pipe, Reference, Pointer, Dereference, Address, Arrow, Ampersand, Asterisks,
  Comment,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Keyword {
  True, False, If, Else, Function, Jump, Async, Await, Return,
  New, Delete, Sizeof, // sizeof is special because it is used as a function!
  Const, Let, Static, For, While, Switch, Case, Default, Continue, Break,
  Import, Export, From, As, Struct, Enum, Namespace, Public, Private, Protected, Abstract,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
  Void, Char, Byte, WideChar, Size, SignedSize, Signed8, Signed16, Signed32, Signed64,
  Unsigned8, Unsigned16, Unsigned32, Unsigned64, Float32, Float64, Bool,
}

lazy_static! {
  pub static ref SYMBOLS: HashMap<&'static str, Symbol> = {
    HashMap::from([
      ("+", Symbol::Plus), ("-", Symbol::Minus), ("/", Symbol::Div), ("%", Symbol::Mod),
      ("+=", Symbol::PlusEquals), ("-=", Symbol::MinusEquals), ("*=", Symbol::MulEquals), ("/=", Symbol::DivEquals), ("%=", Symbol::ModEquals),
      ("++", Symbol::Increment), ("--", Symbol::Decrement),
      ("=", Symbol::Assign), ("==", Symbol::Equals), ("!=", Symbol::NotEquals),
      (">", Symbol::GreaterThan), ("<", Symbol::LessThan), (">=", Symbol::GreaterThanEqual), ("<=", Symbol::LessThanEqual),
      ("(", Symbol::OpenParenthesis), (")", Symbol::CloseParenthesis),
      ("[", Symbol::OpenBracket), ("]", Symbol::CloseBracket),
      ("{", Symbol::OpenBrace), ("}", Symbol::CloseBrace),
      ("!", Symbol::Not), ("&&", Symbol::And), ("||", Symbol::Or),
      ("~", Symbol::BitNot), ("^", Symbol::BitXor),
      ("<<", Symbol::BitLeftShift), (">>", Symbol::BitRightShift),
      ("~=", Symbol::BitNotEquals), ("&=", Symbol::BitAndEquals), ("|=", Symbol::BitXorEquals),
      ("<<=", Symbol::BitLeftShiftEquals), (">>=", Symbol::BitRightShiftEquals),
      (".", Symbol::Dot), (",", Symbol::Comma), ("?", Symbol::Question), (":", Symbol::Colon), (";", Symbol::Semicolon), ("\\", Symbol::Backslash), ("#", Symbol::Hashtag), ("@", Symbol::At), ("::", Symbol::NamespaceAccess),
      ("->", Symbol::Arrow), ("...", Symbol::Variadic),
      ("&", Symbol::Ampersand), ("*", Symbol::Asterisks), ("//", Symbol::Comment), ("|", Symbol::Pipe), ("$", Symbol::Dollar),
    ])
  };

  pub static ref KEYWORDS: HashMap<&'static str, Keyword> = {
    HashMap::from([
      ("if", Keyword::If), ("else", Keyword::Else), ("ret", Keyword::Return), ("new", Keyword::New), ("delete", Keyword::Delete), ("sizeof", Keyword::Sizeof),
      ("fun", Keyword::Function), ("jump", Keyword::Jump), ("async", Keyword::Async), ("await", Keyword::Await),
      ("const", Keyword::Const), ("let", Keyword::Let), ("static", Keyword::Static),
      ("for", Keyword::For), ("while", Keyword::While), ("switch", Keyword::Switch), ("case", Keyword::Case), ("default", Keyword::Default), ("continue", Keyword::Continue), ("break", Keyword::Break),
      ("import", Keyword::Import), ("export", Keyword::Export), ("from", Keyword::From), ("as", Keyword::As),
      ("struct", Keyword::Struct), ("enum", Keyword::Enum), ("namespace", Keyword::Namespace),
      ("public", Keyword::Public), ("private", Keyword::Private), ("protected", Keyword::Protected), ("abstract", Keyword::Abstract),
    ])
  };

  pub static ref TYPES: HashMap<&'static str, Type> = {
    HashMap::from([
      ("void", Type::Void),
      ("char", Type::Char), ("byte", Type::Byte), ("wide_char", Type::WideChar),
      ("size", Type::Size), ("isize", Type::SignedSize),
      ("int8", Type::Signed8), ("int16", Type::Signed16), ("int32", Type::Signed32), ("int64", Type::Signed64),
      ("uint8", Type::Unsigned8), ("uint16", Type::Unsigned16), ("uint32", Type::Unsigned32), ("uint64", Type::Unsigned64),
      ("float32", Type::Float32), ("float64", Type::Float64),
      ("bool", Type::Bool),
    ])
  };

  pub static ref STR_ESCAPE_CODES: HashMap<char, char> = {
    HashMap::from([
      ('n', '\n'), ('r', '\r'), ('t', '\t'),
      ('0', '\0')
    ])
  };

  pub static ref SYMBOL_TERMINATING_CHARS: HashSet<char> = {
    HashSet::from([ '\n', '"', '\'', '\0', '_', ' ' ])
  };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Location {
  pub row: i32,
  pub col: i32
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueVariant {
  Symbol(Symbol),
  Keyword(Keyword),
  Type(Type),
  Integer(i64),
  Decimal(f64),
  Boolean(bool),
  Identifier(String),
  String(String),
  Character(char)
}

#[derive(Debug)]
pub struct Token {
  pub value: ValueVariant,
  pub start: Location,
  pub end: Location
}

pub struct Lexer {
  pub tokens: Vec<Token>,
  location: Location,
  pub token_start: Location
}

#[derive(PartialEq)]
enum LexicalState {
  Any, FindNewLine, Symbol, Identifier, String, Char, Integer, Decimal
}

#[allow(unused_assignments)]
impl Lexer {
  pub fn new() -> Self {
    Lexer {
      tokens: Vec::new(),
      location: Location {
        row: 0,
        col: -1
      },
      token_start: Location {
        row: 0,
        col: 0
      }
    }
  }

  pub fn analyze(&mut self, code: &String) -> Result<(), String> {
    self.tokens.reserve(2000);

    let mut lexical_state = LexicalState::Any;

    let mut accumulator = String::new();
    let mut int_accumulator: i64 = 0;
    let mut decimal_accumulator: f64 = 0.0;
    let mut decimal_place = 10;
    let mut sign = 1;

    let mut back = false;
    let mut next_line = false;
    let mut str_escaped = false;

    let mut i: isize = -1;
    let code_len = code.len() as isize;
 
    'main: while i < code_len {
      i += 1;

      let character = if i != code_len { code.chars().nth(i as usize).unwrap() } else { '\0' };
      
      if next_line {
        self.location.row += 1;
        self.location.col = -1;
        next_line = false;
      }

      if !back {
        if character == '\n' { next_line = true }
        self.location.col += 1;
      } else { back = false }

      if lexical_state == LexicalState::Any {
        if character == ' ' || character == '\n' { continue; }
        else if character == '_' || character.is_ascii_alphabetic() { lexical_state = LexicalState::Identifier; }
        else if character.is_ascii_digit() { lexical_state = LexicalState::Integer }
        else if character == '"' { lexical_state = LexicalState::String; self.token_start = self.location; continue; }
        else if character == '\'' { lexical_state = LexicalState::Char; self.token_start = self.location; continue; }
        else { lexical_state = LexicalState::Symbol; }

        if lexical_state != LexicalState::Integer { accumulator.push(character); }
        else { int_accumulator += (character as u8 - b'0') as i64; }

        self.token_start = self.location;

        continue;
      }

      match lexical_state {
        LexicalState::FindNewLine => {
          if character == '\n' {
            lexical_state = LexicalState::Any;
            continue;
          }
        },
        LexicalState::Symbol => {
          if accumulator == "-" && self.tokens.len() > 0 && character.is_ascii_digit() {
            // If a number is before the -, use the MINUS operator instead of a negative number.
            let variant = self.tokens.last().unwrap().value.clone();

            match variant {
              ValueVariant::Integer(_) | ValueVariant::Decimal(_) | ValueVariant::Identifier(_) => {
                sign = -1;
                lexical_state = LexicalState::Integer;
                accumulator.clear();
                continue;
              },
              _ => {}
            }
          }

          if character.is_ascii_alphanumeric() || SYMBOL_TERMINATING_CHARS.contains(&character) {
            lexical_state = LexicalState::Any;
            let mut slice_pos = 0;
            let mut slice_len = accumulator.len();

            while slice_len > 0 {
              let slice = &accumulator[slice_pos..slice_pos + slice_len];

              if let Some(symbol) = SYMBOLS.get(slice) {
                if *symbol == Symbol::Comment {
                  lexical_state = LexicalState::FindNewLine;
                  accumulator.clear();
                  continue 'main;
                }

                if *symbol == Symbol::Dot && character.is_ascii_digit() {
                  lexical_state = LexicalState::Decimal;
                  accumulator.clear();
                  decimal_accumulator = (character as u8 - b'0') as f64 / 10.0;
                  continue 'main;
                }

                self.tokens.push(Token {
                  value: ValueVariant::Symbol(*symbol),
                  start: self.token_start,
                  end: Location { row: self.location.row, col: self.location.col - 1 }
                });

                slice_pos += slice_len;
                slice_len = accumulator.len() - slice_pos;
                continue;
              }

              slice_len -= 1;
            }

            if slice_pos != accumulator.len() {
              return Err(format!("Unknown symbol: {}.", accumulator));
            }

            accumulator.clear();
            i -= 1;
            back = true;
            continue;
          }

          accumulator.push(character);
        }
        LexicalState::Identifier => {
          if character == '_' || character.is_ascii_alphanumeric() {
            accumulator.push(character);
          } else {
            lexical_state = LexicalState::Any;
            let token_end = Location {
              row: self.location.row,
              col: self.location.col - 1
            };

            if let Some(keyword) = KEYWORDS.get(&accumulator as &str) {
              self.tokens.push(Token {
                value: ValueVariant::Keyword(*keyword),
                start: self.token_start,
                end: token_end
              });
            } else if let Some(token_type) = TYPES.get(&accumulator as &str) {
              self.tokens.push(Token {
                value: ValueVariant::Type(*token_type),
                start: self.token_start,
                end: token_end
              });
            } else if accumulator == "true" || accumulator == "false" {
              self.tokens.push(Token {
                value: ValueVariant::Boolean(accumulator == "true"),
                start: self.token_start,
                end: token_end
              });
            } else {
              self.tokens.push(Token {
                value: ValueVariant::Identifier(accumulator.clone()),
                start: self.token_start,
                end: token_end
              });
            }

            accumulator.clear();
            i -= 1;
            back = true;
          }
        },
        LexicalState::String | LexicalState::Char => {
          if str_escaped {
            if let Some(escape_code) = STR_ESCAPE_CODES.get(&character) {
              accumulator.push(*escape_code);
            } else { accumulator.push(character) }

            str_escaped = false;
          } else {
            if character == '\\' {
              str_escaped = true;
              continue;
            }

            if (lexical_state == LexicalState::String && character == '"') || lexical_state == LexicalState::Char && character == '\'' {
              if lexical_state == LexicalState::Char && accumulator.len() > 1 {
                return Err(String::from("Cannot fit multiple characters in the char type."));
              }

              let token_end = self.location;
              if lexical_state == LexicalState::Char {
                self.tokens.push(Token {
                  value: ValueVariant::Character(accumulator.chars().nth(0).unwrap()),
                  start: self.token_start,
                  end: token_end
                });
              } else {
                self.tokens.push(Token {
                  value: ValueVariant::String(accumulator.clone()),
                  start: self.token_start,
                  end: token_end
                });
              }

              lexical_state = LexicalState::Any;
              accumulator.clear();
            }
            else { accumulator.push(character); }
          }
        },
        LexicalState::Integer => {
          if character.is_ascii_digit() {
            let digit = character as u8 - b'0';
            int_accumulator = int_accumulator * 10 + digit as i64;
          } else if character == '.' {
            lexical_state = LexicalState::Decimal;
            decimal_accumulator = int_accumulator as f64;
          } else {
            lexical_state = LexicalState::Any;
            self.tokens.push(Token {
              value: ValueVariant::Integer(int_accumulator * sign),
              start: self.token_start,
              end: Location {
                row: self.location.row,
                col: self.location.col - 1
              }
            });

            int_accumulator = 0;
            sign = 1;
            i -= 1;
            back = true;
          }
        },
        LexicalState::Decimal => {
          if character.is_ascii_digit() {
            let digit = character as u8 - b'0';
            decimal_accumulator += digit as f64 / decimal_place as f64;
            decimal_place *= 10;
          } else {
            lexical_state = LexicalState::Any;
            self.tokens.push(Token {
              value: ValueVariant::Decimal(decimal_accumulator * sign as f64),
              start: self.token_start,
              end: Location {
                row: self.location.row,
                col: self.location.col - 1
              }
            });

            decimal_accumulator = 0.0;
            decimal_place = 10;
            i -= 1;
            back = true;
          }
        }
        _ => {
          panic!("???");
        }
      }
    }

    Ok(())
  }
}