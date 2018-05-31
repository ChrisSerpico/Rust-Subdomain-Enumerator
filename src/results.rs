use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Results {
    store: Vec<Arc<Mutex<HashSet<String>>>>,
}

impl Query {
    pub fn new(num_domains: usize) -> Self {
        Results{
        	store: vec![Arc::new(Mutex::new(HashSet::new())); num_domains],
        }
    }

    pub fn get_store(&self) -> Vec<Arc<Mutex<HashSet<String>>>>{
    	self.store
    }

    pub fn get_writable_store(&self) -> HashSet<String> {
    	self.store.lock().unwrap()
    }
}