# Lexer

A lexer written in Rust.
The file `example.vit` consists of (some) of the syntax that this lexer targets. The syntax can be changed by modifying the HashMap's in the `src/lexer.rs` file. The lexer will work on other coding languages, however, any code that uses the single quote and doesn't treat it purely as a char, will result in a syntax error. For example, `&'static` is an error because it's an unclosed char. Another invalid char would be `'ti'`. Attempting to use the single quote incorrectly will result in the following error: `Cannot fit multiple characters in the char type.`. A valid char would look like `'t'`.
<br><br>
The highlighting color theme is based off the <b>One Dark Pro</b> theme.

## Running

The target file to lex is passed through command arguments. Example: `cargo run example.vit`. It will then print the file with syntax highlighting.