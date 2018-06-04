use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Results {
	pub domain_list: Vec<String>,
    pub store: Vec<Arc<Mutex<HashSet<String>>>>,
}

impl Results {
    pub fn new(num_domains: usize, list : Vec<String>) -> Self {
        Results{
        	domain_list: list,
        	store: vec![Arc::new(Mutex::new(HashSet::new())); num_domains],
        }
    }

    pub fn insert_subdomain(&self, domain_position: usize, subdomain: String){
    	self.store[domain_position].lock().unwrap().insert(subdomain);
    }

    pub fn print_subdomains(&self){
    	for i in 0..self.domain_list.len() {
    		println!("Domain: {}\n Subdomains:", self.domain_list[i]);
    		for subdomain in self.store[i].lock().unwrap().iter(){
    			println!("{}", subdomain);
    		}
    	}
    }

}