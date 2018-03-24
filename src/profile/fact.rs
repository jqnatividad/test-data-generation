//! The `profile` module provides functionality to create a profile on a data sample (Strings) based on the following attributes:
//! 
//! - symbolic patterns</br>
//! - lengths</br>
//! - pattern probability</br>
//! - Markov chains</br>
//!
//! Once a profile has been made, data can be generated by calling the _pre_generate()_ and _generate()_ functions, in that order.
//!
//! # Example
//!
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::profile::fact::Fact;
//! 
//! fn main() {
//!     //fact created for the character 'r' in the string "word"
//!    	let mut fact =  Fact::new('r','c',0,0,2);
//!
//!     // set the char that appears after the 'r'
//!     fact.set_next_key('d');
//!
//!     // set the char that appears before the 'r'
//!     fact.set_prior_key('o');
//! }
//! ```

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
/// Represents a Fact for a character in a sample data entity that has been analyzed
pub struct Fact{
	/// the char that the fact defines (.e.g: 'a', '1', '%', etc.)
	pub key: char,
	/// the char that appears before (-1) the key in the entity
	pub	prior_key: Option<char>,
	/// the char that appears after (+1) the key in the entity
	pub	next_key: Option<char>,
	/// the PatternPlaceholder symbol that represents the type of key
	pub	pattern_placeholder: char,
	/// indicates if the key is the first char in the entity (0=no, 1=yes)
	pub	starts_with: u32,
	/// indicates if the key is the last char in the entity (0=no, 1=yes)
	pub	ends_with: u32,
	/// indicates the number of positions from the index zero (where the char is located in the entity from the first position)
	pub	index_offset: u32,
}

impl Fact {
	/// Constructs a new Fact
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::fact::Fact;
	///	
	/// fn main() {
	/// 	//fact created for the character 'r' in the string "word"
    ///    	let mut fact =  Fact::new('r','c',0,0,2);
	/// }
	/// ```
	pub fn new(k: char, pp: char, sw: u32, ew: u32, idx_off: u32 ) -> Fact {
		Fact{
			key: k,
			prior_key: None,
			next_key: None,
			pattern_placeholder: pp,
			starts_with: sw,
			ends_with: ew,
			index_offset: idx_off,
		}
	}
	
	/// Constructs a new Fact from a serialized (JSON) string of the Fact object. This is used when restoring from "archive"
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::fact::Fact;
	///	
	/// fn main() {	
	///		let serialized = "{\"key\":\"r\",\"prior_key\":null,\"next_key\":null,\"pattern_placeholder\":\"c\",\"starts_with\":0,\"ends_with\":0,\"index_offset\":2}";
    ///		let mut fact = Fact::from_serialized(&serialized);
    ///     fact.set_prior_key('a');
    ///		fact.set_next_key('e');
    ///
    ///		assert_eq!(fact.pattern_placeholder, 'c');
	/// }    	
    /// ```	
	pub fn from_serialized(serialized: &str) -> Fact {
		serde_json::from_str(&serialized).unwrap()
	}
	
	/// This function converts the Fact to a serialize JSON string.
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::fact::Fact;
	///	
	/// fn main() {
	/// 	//fact created for the character 'r' in the string "word"
    ///    	let mut fact =  Fact::new('r','c',0,0,2);
    ///
    ///     println!("{}", fact.serialize());
    ///     // {"key":"r","prior_key":null,"next_key":null,"pattern_placeholder":"c","starts_with":0,"ends_with":0,"index_offset":2}
	/// }
	/// 	
	pub fn serialize(&mut self) ->String {
		serde_json::to_string(&self).unwrap()
	}
	
	/// This function sets the next key attribute to the specified char.
	/// 
	/// # Arguments
	///
	/// * `nk: char` - The character that represents the next character in the entity
	///
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::fact::Fact;
	///	
	/// fn main() {
	/// 	//fact created for the character 'r' in the string "word"
    ///    	let mut fact =  Fact::new('r','c',0,0,2);
    ///     fact.set_next_key('d');
	/// }
	/// 
	pub fn set_next_key(&mut self, nk: char) {
		self.next_key = Some(nk);
	}
	
	/// This function sets the prior key attribute to the specified char.
	/// 
	/// # Arguments
	///
	/// * `pk: char` - The character that represents the prior character in the entity
	///
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::fact::Fact;
	///	
	/// fn main() {
	/// 	//fact created for the character 'r' in the string "word"
    ///    	let mut fact =  Fact::new('r','c',0,0,2);
    ///     fact.set_prior_key('o');
	/// }
	/// 	
	pub fn set_prior_key(&mut self, pk: char) {
		self.prior_key = Some(pk);
	}
}