mod parser;

use parser::Lexer; // tem que importar toda a vez que for usar JsLexer?? Aff..

fn main() {

	/*
	let js_lexer = parser::JsLexer{
		token_pool: vec![] // FIXME ver como colocar isso no construtor do Lexer quem sabe
	};
	*/
	let mut js_lexer:parser::JsLexer = Default::default();
	let tokens = js_lexer.parse("let a = 'a';");
	println!("output:");
	for token in &tokens {
		println!("{}", token);
	}

}