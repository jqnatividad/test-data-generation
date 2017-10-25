//! The `profile` module provides functionality to create a profile on a data sample (Strings) based on the following attributes:
//! 
//! - symbolic patterns</br>
//! - lengths</br>
//! - pattern probability</br>
//! - Markov chains</br>
//!
//! Once a profile has been made, data can be generated by calling the _pre_generate()_ and _generate()_ functions, in that order.
//!
//! # Examples
//!
//! Profiling some sample data...
//! 
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::profile::profile::Profile;
//! 
//! fn main() {
//!     // analyze the dataset
//!		let mut data_profile =  Profile::new();
//!
//!     // analyze the dataset
//!		data_profile.analyze("Smith, John");
//!		data_profile.analyze("Doe, John"); 
//!		data_profile.analyze("Dale, Danny"); 
//!		data_profile.analyze("Rickets, Ronney");
//!
//!     // confirm 4 data samples were analyzed   		
//!    	assert_eq!(data_profile.patterns.len(), 4);
//! }
//! ```
//!
//! Generating a person's name from some sample data...
//! 
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::profile::profile::Profile;
//! 
//! fn main() {
//!		let mut data_profile =  Profile::new();
//!
//!     // analyze the dataset
//!		data_profile.analyze("Smith, John");
//!		data_profile.analyze("Doe, John"); 
//!		data_profile.analyze("Dale, Danny"); 
//!		data_profile.analyze("Rickets, Ronney"); 
//!		data_profile.analyze("Taylor, Mike"); 
//!		data_profile.analyze("Bates, Betty"); 
//!
//!     // prepare the generator
//!     data_profile.pre_generate();
//!    	 		
//!     // generate some data
//!    	println!("The generated name is {:?}", data_profile.generate());
//! }
//! ```
//!
//! Generating a date from some sample data...
//! 
//! ```
//! extern crate test_data_generation;
//!
//! use test_data_generation::profile::profile::Profile;
//! 
//! fn main() {
//!		let mut data_profile =  Profile::new();
//!
//!     // analyze the dataset
//!		data_profile.analyze("01/01/2017");
//!		data_profile.analyze("01/02/2017"); 
//!		data_profile.analyze("01/03/2017"); 
//!		data_profile.analyze("01/04/2017"); 
//!		data_profile.analyze("01/05/2017");  
//!		data_profile.analyze("01/06/2017");  
//!		data_profile.analyze("01/07/2017");  
//!		data_profile.analyze("01/08/2017");  
//!		data_profile.analyze("01/09/2017"); 
//!		data_profile.analyze("01/10/2017");  
//!
//!     // prepare the generator
//!     data_profile.pre_generate();
//!    	 		
//!     // generate some data
//!    	println!("The generated date is {:?}", data_profile.generate());
//! }
//! ```

use profile::pattern::{Pattern};
use profile::fact::{Fact};
use std::collections::BTreeMap;
use std::ops::AddAssign;
use crossbeam;

type PatternMap = BTreeMap<String, u32>;
type SizeMap = BTreeMap<u32, u32>;
type SizeRankMap  = BTreeMap<u32, f64>;

/// Represents a Profile for sample data that has been analyzed and can be used to generate realistic data
pub struct Profile {
	/// A list of symbolic patterns with a distinct count of occurrences
	pub patterns: PatternMap,
	/// The total number of patterns in the profile
	pub pattern_total: u32,
	/// A list of symbolic patterns in the profile 
	/// (used for temporary storage due to lifetime issues)
	pub pattern_keys: Vec<String>,
	/// A list of distinct counts for patterns in the profile 
	/// (used for temporary storage due to lifetime issues)
	pub pattern_vals: Vec<u32>,
	/// A list of symbolic patterns with their percent chance of occurrence
	pub pattern_percentages: Vec<(String, f64)>,
	/// A list of symbolic patterns with a running total of percent chance of occurrence, in increasing order
	pub pattern_ranks: Vec<(String, f64)>,
	/// A list of pattern lengths with a distinct count of occurrence
	pub sizes: SizeMap,
	/// the total number of pattern sizes (lengths) in the profile
	pub size_total: u32,
	/// A list of pattern sizes (lengths) with a running total of their percent chance of occurrence, in increasing order
	pub size_ranks: Vec<(u32, f64)>,
	/// The number of processors used to distribute the work load (multi-thread) while finding Facts to generate data
	pub processors: u8,
	/// A list of processors (which are lists of Facts) that store all the Facts in the profile
	pub facts: Vec<Vec<Fact>>,
}

impl Profile {
	/// Constructs a new Profile
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::profile::Profile;
	///	
	/// fn main() {
	/// 	let placeholder = Profile::new();
	/// }
	/// ```
	pub fn new() -> Profile {
		Profile {
			patterns: PatternMap::new(),
			pattern_total: 0,
			pattern_keys: Vec::new(),
			pattern_vals: Vec::new(),
			pattern_percentages: Vec::new(),
			pattern_ranks: Vec::new(),
			sizes: SizeMap::new(),
			size_total: 0,
			size_ranks: Vec::new(), 
			processors: 4,
			facts: Profile::new_facts(4),
		}
	}

	/// Constructs a new Profile with a specified number of processors to analyze the data.
	/// Each processor shares the load of generating the data based on the Facts it has been assigned to manage.
	/// 
	/// # Arguments
	///
	/// * `p: u8` - A number that sets the number of processors to start up to manage the Facts.</br>
	///         Increasing the number of processors will speed up the generator be ditributing the workload.
	///         The recommended number of processors is 1 per 10K data points (e.g.: profiling 20K names should be handled by 2 processors)</br>
	///         NOTE: The default number of processors is 4.
	/// 
	/// #Example
	/// 
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::profile::Profile;
	///	
	/// fn main() {
	///     let processors: u8 = 10;
	/// 	let placeholder = Profile::new_with(processors);
	/// }
	/// ```	
	pub fn new_with(p: u8) -> Profile {
		Profile {
			patterns: PatternMap::new(),
			pattern_total: 0,
			pattern_keys: Vec::new(),
			pattern_vals: Vec::new(),
			pattern_percentages: Vec::new(),
			pattern_ranks: Vec::new(),
			sizes: SizeMap::new(),
			size_total: 0,
			size_ranks: Vec::new(), 
			processors: p,
			facts: Profile::new_facts(p),
		}
	}

	/// This function converts an data point (&str) to a pattern and adds it to the profile
	/// 
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::profile::Profile;
	/// 
	/// fn main() {
    /// 	let mut profile =  Profile::new();
    ///		profile.analyze("One");
    ///		profile.analyze("Two"); 
    ///		profile.analyze("Three"); 
    ///		profile.analyze("Four"); 
    ///	    		
    ///		assert_eq!(profile.patterns.len(), 4);
	/// }
	/// ```	
	pub fn analyze(&mut self, entity: &str) {
		let mut pattrn =  Pattern::new();
		
		// analyze patterns
		let rslt = pattrn.analyze(entity);
		
		// balance the storing of facts across all the vectors that can be processed in parallel
		let mut i = 0;
		for f in rslt.1.into_iter() {			
			if i == self.processors {
				i = 0;
			}

			self.facts[i as usize].push(f);
			i = i + 1;
			
		}
		
		// store the pattern
		AddAssign::add_assign(self.patterns.entry(rslt.0.to_string()).or_insert(0), 1);
		
		// store the total number of patterns generated so far
		self.pattern_total = self.patterns.values().sum::<u32>();
		
		// analyze sizes
		AddAssign::add_assign(self.sizes.entry(pattrn.size).or_insert(0), 1);
		self.size_total = self.sizes.values().sum::<u32>();
		
		self.pattern_keys = self.patterns.keys().cloned().collect();
		self.pattern_vals = self.patterns.values().cloned().collect();
	} 
	
	/// This function generates realistic test data based on the sample data that was analyzed.
	/// 
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::profile::Profile;
	/// 
	/// fn main() {
    /// 	let mut profile =  Profile::new();
    ///
    ///		profile.analyze("01/13/2017");
    ///		profile.analyze("11/24/2017"); 
    ///		profile.analyze("08/05/2017");
    ///	    		
    ///     profile.pre_generate();
    ///
    ///  	let generated = profile.apply_facts("##p##p####".to_string());
    ///
    ///     assert_eq!(generated.len(), 10);
    /// }
	/// ```	
	pub fn apply_facts(&self, pattern: String) -> String {
		let pattern_chars = pattern.chars().collect::<Vec<char>>();
		let mut generated = String::new();
		let mut prev_char = ' ';
	
		// iterate through the chars in the pattern string
		for (idx, ch) in pattern_chars.iter().enumerate() {
			//println!("pattern_chars index: {:?}",idx);	
			//println!("prev_char{:?}",prev_char);
					
			crossbeam::scope(|scope| {
				let c = ch;
				let starts = if idx == 0 { 1 } else { 0 };
			 	let ends = if idx == pattern_chars.len()-1 { 1 } else { 0 };
			 	let mut fact_options = vec![];
			 	let prior_char = prev_char;
		 	
			 	// iterate through the processors (vec) that hold the lists (vec) of facts
				for v in &self.facts {
					//println!("list number {:?}", v.len());
					let selected_facts = scope.spawn(move || {	
						let mut facts = vec![];		
					
						// iterate through the list of facts				
						for value in v {
							if value.starts_with == starts && 
							   value.ends_with == ends && 
							   value.pattern_placeholder == *c && 
							   value.index_offset == idx as u32 {
									facts.push(value.key.clone());
								
									// if the value.key's prior char matches the prior generated char, then weight the value.key 
									// to increase the chance of it being used when generated
									if value.prior_key.unwrap_or(' ') == prior_char {
										facts.push(value.key.clone());
										facts.push(value.key.clone());
									}
									
									// if the value.key's index_offset matches the current index, then weight the value.key 
									// to increase the chance of it being used when generated
									if value.index_offset == idx as u32 {
										facts.push(value.key.clone());
										facts.push(value.key.clone());
									}
							}
						}
						
						facts
					});					
					//println!("list of selected facts for [{:?}] : {:?}",ch, selected_facts.join());
					fact_options.extend_from_slice(&selected_facts.join());					
				}
				
				//select a fact to use as the generated char
				//println!("list of selected facts for [{:?}] : {:?}",ch,fact_options);
				
				let mut x:u32 = 0;
				let rnd_start = 0;
				let rnd_end = fact_options.len()-1;
				
				if rnd_start >= rnd_end {
					generated.push(fact_options[0 as usize]);
				}else{
					random_between!(x, rnd_start, rnd_end);
					//println!("{:?}",fact_options[x as usize]);
					prev_char = fact_options[x as usize];
					generated.push(prev_char);
				}
			}); 		
		}
	
		//println!("The generated value is.. {:?}", generated);
		generated
	}
	
	/// This function calculates the patterns to use by the chance they will occur (as cumulative percentage) in decreasing order
	/// 
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::profile::Profile;
	/// 
	/// fn main() {
    /// 	let mut profile =  Profile::new();
    ///
    ///    	profile.analyze("Smith, John");
    ///    	profile.analyze("O'Brian, Henny"); 
    ///    	profile.analyze("Dale, Danny"); 
    ///    	profile.analyze("Rickets, Ronnae"); 
    ///    	profile.analyze("Richard, Richie");
    ///    	profile.analyze("Roberts, Blake");
    ///    	profile.analyze("Conways, Sephen");
    ///    	
    ///    	profile.pre_generate();	
    ///    	let test = [("CvccvccpSCvccvv".to_string(), 28.57142857142857 as f64), ("CcvccpSCvcc".to_string(), 42.857142857142854 as f64), ("CvccvccpSCvccvc".to_string(), 57.14285714285714 as f64), ("CvcvcccpSCcvcv".to_string(), 71.42857142857142 as f64), ("CvcvpSCvccc".to_string(), 85.7142857142857 as f64), ("V@CcvvcpSCvccc".to_string(), 99.99999999999997 as f64)];    		
    ///    	    		
    ///    	assert_eq!(profile.pattern_ranks, test);
    /// }
	/// ```	
	pub fn cum_patternmap(&mut self) {
		// Reference: https://users.rust-lang.org/t/cannot-infer-an-appropriate-lifetime-for-autoref/13360/3
			
		// calculate the percentage by patterns
		// -> {"CcvccpSCvcc": 14.285714285714285, "CvccvccpSCvccvc": 14.285714285714285, "CvccvccpSCvccvv": 28.57142857142857, "CvcvcccpSCcvcv": 14.285714285714285, "CvcvpSCvccc": 14.285714285714285, "V~CcvvcpSCvccc": 14.285714285714285}	
		let n = self.patterns.len();
		
		for m in 0..n {
			self.pattern_percentages.push((self.pattern_keys[m].clone(), (self.pattern_vals[m] as f64 / self.pattern_total as f64) * 100.0));
		}

		// sort the ranks by percentages in decreasing order
		// -> [("CvccvccpSCvccvv", 28.57142857142857), ("CcvccpSCvcc", 14.285714285714285), ("CvccvccpSCvccvc", 14.285714285714285), ("CvcvcccpSCcvcv", 14.285714285714285), ("CvcvpSCvccc", 14.285714285714285), ("V~CcvvcpSCvccc", 14.285714285714285)]
		self.pattern_percentages.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());

		// calculate the cumulative sum of the pattern rankings
		// -> [("CvccvccpSCvccvv", 28.57142857142857), ("CcvccpSCvcc", 42.857142857142854), ("CvccvccpSCvccvc", 57.14285714285714), ("CvcvcccpSCcvcv", 71.42857142857142), ("CvcvpSCvccc", 85.7142857142857), ("V~CcvvcpSCvccc", 99.99999999999997)] 
		let mut rank: f64 = 0.00;
		
		for pttrn in self.pattern_percentages.iter() {
			let tmp = pttrn.1 + rank;
			self.pattern_ranks.push((pttrn.0.clone(),tmp));
			rank = tmp;
		}
	}
	
    /// This function calculates the sizes to use by the chance they will occur (as cumulative percentage) in decreasing order
	/// 
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::profile::Profile;
	/// 
	/// fn main() {
    /// 	let mut profile =  Profile::new();
    ///		profile.analyze("One");
    ///		profile.analyze("Two"); 
    ///		profile.analyze("Three"); 
    ///		profile.analyze("Four");  
    ///		profile.analyze("Five");
    ///		profile.analyze("Six");
    ///	    		
    ///     profile.cum_sizemap();
    ///
    ///		print!("The size ranks are {:?}", profile.size_ranks);
    ///     // The size ranks are [(3, 50), (4, 83.33333333333333), (5, 100)] 
    /// }
	/// ```	
	pub fn cum_sizemap(&mut self) {
		// calculate the percentage by sizes
		// -> {11: 28.57142857142857, 14: 14.285714285714285, 15: 57.14285714285714}
		let mut size_ranks = SizeRankMap::new();
		
		for key in self.sizes.keys(){
			size_ranks.insert(*key, (*self.sizes.get(key).unwrap() as f64 / self.size_total as f64)*100.0);
		}
	
		// sort the ranks by percentages in decreasing order
		// -> [(15, 57.14285714285714), (11, 28.57142857142857), (14, 14.285714285714285)]
		let mut sizes = size_ranks.iter().collect::<Vec<_>>();
		sizes.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());
		
		// calculate the cumulative sum of the size rankings
		// -> [(15, 57.14285714285714), (11, 85.71428571428571), (14, 100)]
		self.size_ranks = sizes.iter().scan((0 as u32, 0.00 as f64), |state, &(&k, &v)| {
			*state = (k, state.1 + &v);
			Some(*state)
		}).collect::<Vec<(_,_)>>();	
	}
	
	/// This function generates realistic test data based on the sampel data that was analyzed.
	/// 
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::profile::Profile;
	/// 
	/// fn main() {
    /// 	let mut profile =  Profile::new();
    ///
    ///		profile.analyze("One");
    ///		profile.analyze("Two"); 
    ///		profile.analyze("Three"); 
    ///		profile.analyze("Four");  
    ///		profile.analyze("Five");
    ///	    		
    ///     profile.pre_generate();
    ///
    ///		print!("The test data {:?} was generated.", profile.generate());
    /// }
	/// ```	
	pub fn generate(&mut self) -> String{
		// 1. get a random number
	 	let mut s: f64 = 0 as f64;
	 	random_percentage!(s);
	 	
	 	// 2. find the first pattern that falls within the percentage chance of occurring
	 	// NOTE: The following 2 lines has been commented out because this doesn't need to 
	 	//       happen since the patterns are already ranks by percent chance of occurring 
	 	//       and therefore sizes (lengths) as well since the patterns include the full 
	 	//       length of the entitiy analyzed.
		//let size = self.size_ranks.iter().find(|&&x|&x.1 >= &s).unwrap().0;	 	
		//let pattern = self.pattern_ranks.iter().find(|x|&x.1 >= &s && x.0.len() == size as usize).unwrap().clone();
		let pattern = self.pattern_ranks.iter().find(|x|&x.1 >= &s).unwrap().clone();		

		// lastly, generate the test data using facts that adhere to the pattern 
		let generated = self.apply_facts(pattern.0);
	
		generated
	}
	
	/// This function is called from within the implementated structure and returns a list processors (Vec) with empty lists (Vec) for their Facts.
	/// Each processor shares the load of generating the data based on the Facts it has been assigned to manage.
	/// 
	/// # Arguments
	///
	/// * `p: u8` - A number that sets the number of processors to start up to manage the Facts.</br>
	///         Increasing the number of processors will speed up the generator be ditributing the workload.
	///         The recommended number of processors is 1 per 10K data points (e.g.: profiling 20K names should be handled by 2 processors)</br>
	///         NOTE: The default number of processors is 4.
	/// 
	fn new_facts(p: u8) -> Vec<Vec<Fact>> {
		let mut vec_main = Vec::new();
		
		for _ in 0..p {  
			vec_main.push(Vec::new());
		}

		vec_main
	}
	
	/// This function prepares the size a pattern accumulated percentages order by percentage increasing
	/// 
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::profile::Profile;
	/// 
	/// fn main() {
    /// 	let mut profile =  Profile::new();
    ///		profile.analyze("One");
    ///		profile.analyze("Two"); 
    ///		profile.analyze("Three"); 
    ///		profile.analyze("Four");  
    ///		profile.analyze("Five");
    ///		profile.analyze("Six");
    ///	    		
    ///     profile.pre_generate();
    ///
    ///		print!("The size ranks are {:?}", profile.size_ranks);
    ///     // The size ranks are [(3, 50), (4, 83.33333333333333), (5, 100)] 
    /// }
	/// ```	
	pub fn pre_generate(&mut self){
		self.cum_sizemap();
		self.cum_patternmap();
	}

	/// This function resets the patterns that the Profile has analyzed.
	/// Call this method whenever you wish to "clear" the Profile 
	/// 
	/// # Example
	///
	/// ```
	/// extern crate test_data_generation;
	///
	/// use test_data_generation::profile::profile::Profile;
	/// 
	/// fn main() {
    /// 	let mut profile =  Profile::new();
    ///
    ///		profile.analyze("One");
    ///		profile.analyze("Two"); 
    ///		profile.analyze("Three"); 
    ///    
    ///     let x = profile.patterns.len();
    ///
    ///     profile.reset_analyze();
    ///
    ///		profile.analyze("Four");
    ///		profile.analyze("Five"); 
    ///		profile.analyze("Six");  
    ///		profile.analyze("Seven"); 
    ///		profile.analyze("Eight"); 
    ///		profile.analyze("Nine"); 
    ///		profile.analyze("Ten");
    ///    
    ///     let y = profile.patterns.len();
    ///	    		
    ///     assert_eq!(x, 3);
    ///     assert_eq!(y, 5);
    /// }
	/// ```	
	pub fn reset_analyze(&mut self) {
		self.patterns = PatternMap::new();
	}
}