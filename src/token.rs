//! Módulo do parser

use std::fmt;



#[derive(Default)]
pub struct Token {

	pub token_type:String,

	// TODO ver pra quê serviam essas props
	// openType: {value: '', enumerable: true, writable: true},
	// closeType: {value: '', enumerable: true, writable: true},
	// characterSequence: {value: [], enumerable: true},

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
	is_complete:bool // TODO ver se pode ser o mesmo nome do método, qual a regra de nomeclatura?

}


impl Token {

	pub fn new(token_type:String) -> Token {
		Token {
			token_type: token_type,
			begin: 0,
			end: 0,
			ignore: false,
			is_initialized: false,
			has_next: false,
			is_complete: false
		}
	}

	pub fn has_next(&self) -> bool {
		if self.has_next && !self.is_complete {
			return true;
		}
		return false;
	}

	// TODO implementar via trait
	/*
	pub fn next() {
		//
	}
	*/
	pub fn next(&mut self, source:&str, index:usize) {
		//
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

}



impl Clone for Token {

	fn clone(&self) -> Token {
		Token {
			token_type: self.token_type.clone(),
			begin: self.begin,
			end: self.end,
			ignore: self.ignore,
			is_initialized: self.is_initialized,
			has_next: self.has_next,
			is_complete: self.is_complete
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