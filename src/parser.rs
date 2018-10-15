//! Módulo do parser

#[path = "token.rs"]
mod token;

use self::token::Token;



/// Trait base para todos os lexers
pub trait Lexer {

	/// Lê o código fonte passado e retorna os tokens identificados
	fn parse(&mut self, source:&str) -> Vec<Token> {

		println!("input:");
		println!("{}\n", source);

		let mut token_sequence:Vec<Token> = vec![];

		self.reset_tokens();

		//self.set_index(0);
		let mut index:usize = 0;
		for character in source.chars() {
			// TODO characters.push(character);
			self.iterate(&mut token_sequence, source, index);
			//let current_index = self.index();
			//self.set_index(current_index + 1);
			index = index + 1;
		}

		// characters = END_OF_FILE
		self.iterate(&mut token_sequence, source, index);

		token_sequence

	}

	/// Gera a lista de tokens baseado na identificação dos caracteres
	fn iterate(&mut self, token_sequence:&mut Vec<Token>, source:&str, index:usize) {

		self.match_tokens(source, index);

		let optional_token = self.clean_token_pool();

		if self.token_pool().len() == 0 {

			// TODO
			// vou testar os que não foram testados, mas vou retestar o único que já foi rejeitado
			// tem como melhorar esse reset?
			// por enquanto vou deixar assim
			// podia salvar os tokens que não match numa array e só recolocá-los ao invés de resetar tudo
			if let Some(complete_token) = optional_token {
				token_sequence.push(complete_token);
			}

			self.reset_tokens();
			self.match_tokens(source, index);

		}

	}

	/// Identifica quais são os possíveis tokens para a sequência atual de caracteres
	fn match_tokens(&mut self, source:&str, index:usize) {

		let mut token_pool = self.token_pool();

		for i in token_pool.len()..0 {

			let mut token = &mut token_pool[i];

			if token.has_next() {
				token.next(source, index);
			}

		}

	}

	/// Remove os tokens que não podem mais se enquadrar na sequencia atual de caracteres
	fn clean_token_pool(&mut self) -> Option<Token> {

		let mut token_pool = self.token_pool();

		let mut complete_token:Option<Token> = None;

		for i in token_pool.len()..0 {

			let mut splice = false;

			// escopo para isolar o borrow to token pool e permitir splice dele mais abaixo
			{
				let mut token = &token_pool[i];
				if token.is_complete() {
					complete_token = Some(token.clone());
					break;
				}

				// tira do pool
				if token.is_complete() || token.has_next() {
					splice = true;
				}
			}

			if splice {
				let range_end = i + 1;
				token_pool.splice(i..range_end, vec![]);
			}

		}

		complete_token

	}

	/// Reseta o pool de tokens possíveis para uma nova sequência de caracteres
	fn reset_tokens(&mut self) {
		self.token_pool().clear();
	}

	/// Getter do token pool para a trait
	fn token_pool(&mut self) -> &mut Vec<Token>;

	/// Getter do index para a trait
	fn index(&self) -> usize;

	/// Setter do index para a trait
	fn set_index(&mut self, index:usize);

}



/// Lexer JavaScript
#[derive(Default)]
pub struct JsLexer {

	/// Todos os tokens disponíveis na linguagem
	token_pool:Vec<Token>,

	/// Ponteiro da posição atual do parser
	index:usize

}



impl Lexer for JsLexer {

	/*fn reset_tokens(&mut self) {

		// TODO limpar tokens e preencher de novo

	}*/

	fn token_pool(&mut self) -> &mut Vec<Token> {
		&mut self.token_pool
	}

	fn index(&self) -> usize {
		self.index
	}

	fn set_index(&mut self, index:usize) {
		self.index = index;
	}

}