//! Módulo do token

use std::fmt;



#[derive(Default)]
pub struct Token {

	/// Tipo do token
	pub token_type:String,

	// TODO ver pra quê serviam essas props
	// openType: {value: '', enumerable: true, writable: true},
	// closeType: {value: '', enumerable: true, writable: true},

	// TODO ver se é realmente necessário já que guardo os índices
	/// Sequência de caracteres do token
	pub character_sequence:Vec<char>,

	/// Índice do começo do token
	pub begin:usize,

	/// Índice do fim do token (inclusivo?)
	pub end:usize,

	/// Se o token não participa da validação de sintaxe?
	pub ignore:bool,

	/// ??
	is_initialized:bool,

	/// Se é possível que o token possa ter mais caracteres
	has_next:bool,

	/// Se a sequência atual de caracteres pode ser considerada um token completo
	is_complete:bool, // TODO ver se pode ser o mesmo nome do método, qual a regra de nomeclatura?

	// atributos do Source_simple_character_sequence_token

	/// ?
	previous_matched_keyword:String,

	/// Se a sequência atual de caracteres se enquadra em alguma palavra-chave
	matched_keyword:bool,

	/// Lista com as palavras-chaves que ainda podem se enquadrar na sequência atual de caracteres
	keywords_pool:Vec<String>,

	/// ?
	keyword:String,

	/// ?
	keyword_pointer:usize,

	/// Lista com todas as palavras-chave possíveis
	keywords:Vec<String>

}


impl Token {

	pub fn new(token_type:String) -> Token {
		Token {
			token_type: token_type,
			character_sequence: vec![],
			begin: 0,
			end: 0,
			ignore: false,
			is_initialized: false,
			has_next: false,
			is_complete: false,

			previous_matched_keyword: String::from(""),
			matched_keyword: false,
			keywords_pool: vec![],
			keyword: String::from(""),
			keyword_pointer: 0,
			keywords: vec![]
		}
	}

	pub fn has_next(&self) -> bool {
		if self.has_next && !self.is_complete {
			return true;
		}
		return false;
	}

	// TODO implementar via trait
	pub fn next(&mut self, source:&str, index:usize) {

		let match_character = 'a';

		if !self.match_keywords(match_character) {
			self.has_next = false;
			if self.matched_keyword {
				self.complete();
			}
			return;
		}

		if !self.is_initialized {
			self.is_initialized = true;
			self.begin = index;
		}

		self.character_sequence.push(match_character);

	}

	/// Se a sequência atual de caracteres pode ser considerada um token completo
	pub fn is_complete(&self) -> bool {
		self.is_complete
	}

	/// Conclui o token com os caracteres atuais
	pub fn complete(&mut self) {
		// this.end = this.begin + this.characterSequence.length; // TODO
		self.has_next = false;
		self.is_complete = true;
	}

	// métodos do Source_simple_character_sequence_token (além do next acima)

	fn reset(&mut self) {
		self.matched_keyword = false;
		//self.complete_next_turn = false;
		self.keyword = String::from("");
		self.keyword_pointer = 0;
		self.reset_keywords();
	}

	fn match_keywords(&self, match_character:char) -> bool {

		false

		/*
		// FIXME ver o caso de &lt;&amp;str que ignora o match &lt; pq tem o &lt;&lt;

		if (this._keywordsPool.length > 1) {
			this._previousMatchedKeyword = this._matchedKeyword;
			this._matchedKeyword = null;
		}

		for (let i = this._keywordsPool.length - 1; i > -1; i--) {

			this._keyword = this._keywordsPool[i];
			let character = this._keyword.substr(this._keywordPointer, 1);

			if (character !== matchCharacter) {
				this._keywordsPool.splice(i, 1);
				continue;
			}

			if ((this._keyword.length - 1) === this._keywordPointer) {
				this._matchedKeyword = this._keyword;
			}

		}

		this._keywordPointer += 1;

		if (this._keywordsPool.length > 0) {
			return true;
		} else if (this._previousMatchedKeyword) {
			this._matchedKeyword = this._previousMatchedKeyword;
		}

		return false;
		*/

	}

	fn reset_keywords(&mut self) {
		self.keywords_pool.clear();
		for keyword in &self.keywords {
			self.keywords_pool.push(keyword.clone()); // ou uso lifetimes para não clonar, vale a pena? Keywods nunca vai sofrer alterações
		}
	}

}



impl Clone for Token {

	fn clone(&self) -> Token {
		Token {
			token_type: self.token_type.clone(),
			character_sequence: self.character_sequence.clone(),
			begin: self.begin,
			end: self.end,
			ignore: self.ignore,
			is_initialized: self.is_initialized,
			has_next: self.has_next,
			is_complete: self.is_complete,

			previous_matched_keyword: self.previous_matched_keyword.clone(),
			matched_keyword: self.matched_keyword,
			keywords_pool: self.keywords_pool.clone(),
			keyword: self.keyword.clone(),
			keyword_pointer: self.keyword_pointer,
			keywords: self.keywords.clone()
		}
	}

}



impl fmt::Display for Token {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "token {} {}-{})", self.token_type, self.begin, self.end)
	}

}



/*
impl Iterator for Token {

	type Item = usize; // qual elemento vou retornar

	fn next(&mut self) -> Option<usize> {

		// increment our count. This is why we started at zero.
		self.count += 1;

		// check to see if we've finished counting or not.
		if self.count < 6 {
			Some(self.count)
		} else {
			None
		}

	}

}
*/