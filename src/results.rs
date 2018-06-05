use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Results {
	pub domains:    Vec<String>,
    pub store:      Vec<Arc<Mutex<HashSet<String>>>>,
}

impl Results {
    pub fn new(num_domains: usize, domains_list : Vec<String>) -> Self {
        let mut new_results = Results {
            domains: domains_list,
        	store: Vec::new(),
        };

        for _i in 0..num_domains{
        	new_results.store.push(Arc::new(Mutex::new(HashSet::new())));
        }

        new_results
    }

    pub fn print_subdomains(&self) {
    	for i in 0..self.domains.len() {
    		println!("Domain: {}", self.domains[i]);
    		println!("Subdomains:");
    		for subdomain in self.store[i].lock().unwrap().iter(){
    			println!("\t\t{}", subdomain);
    		}
    	}
    }
}
