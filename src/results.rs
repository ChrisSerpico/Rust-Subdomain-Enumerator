use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Results {
    pub store: Vec<Arc<Mutex<HashSet<String>>>>,
}

impl Results {
    pub fn new(num_domains: usize) -> Self {
        Results{
        	store: vec![Arc::new(Mutex::new(HashSet::new())); num_domains],
        }
    }

    pub fn insert_subdomain(&self, domain_position: usize, subdomain: String){
    	self.store[domain_position].lock().unwrap().insert(subdomain);
    }

    pub fn print_subdomains(&self){
    	println!("{:?}", self.store);



    }

}