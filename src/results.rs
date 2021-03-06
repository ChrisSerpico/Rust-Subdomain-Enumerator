//! Builds the results for subdomain enumeration.
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// A struct holding results of a subdomain enumeration. 
/// domain_list is the list of original domains that were supplied, eg "Facebook.com". 
/// store holds the subdomains found for each supplied domain, eg "messenger.facebook.com". 
/// 
/// # Fields
/// * 'domains' - A Vector of Strings holding the domains that were enumerated 
/// * 'store' - A Vec<Arc<Mutex<HashSet<String>>>> where each element is a list of subdomains corresponding to a domain in domains. 
#[derive(Debug, Clone)]
pub struct Results {
	pub domains:    Vec<String>,
    pub store:      Vec<Arc<Mutex<HashSet<String>>>>,
}

impl Results {

    /// Initiates a new Results instance.
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

    /// Print the associated subdomains found for each domain.
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
