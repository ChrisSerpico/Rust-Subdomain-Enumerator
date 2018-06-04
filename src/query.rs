extern crate reqwest;
extern crate dns_lookup;

use std::sync::Arc;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::mem;
use std::thread;
use enumerator;
use library_enumerator;
use results::Results;

#[derive(Deserialize, Debug)]
struct Resp {
    data: Vec<Subdomain>,
}

#[derive(Deserialize, Debug)]
struct Subdomain {
    id: String,
}


#[derive(Debug, Clone)]
pub struct Query {
    domains: Vec<String>,
    library: String,
    limit: usize,
    num_domains: usize,

}

impl Query {
    pub fn new() -> Self {
        Query {
            domains: Vec::new(),
            library: String::new(),
            limit: 10,
            num_domains: 0,
        }
    }

    pub fn add_domain(&mut self, domain: String){
        self.domains.push(domain);
        self.num_domains += 1;
    }

    pub fn add_domains(&mut self, domains: Vec<String>){
        self.domains = domains;
        self.num_domains = self.domains.len();
    }

    pub fn set_library(&mut self, library: String){
        self.library = library;
    }

    pub fn set_limit(&mut self, limit: usize){
       	self.limit = limit;
    }

    pub fn get_num_domains(&self) -> usize{
    	self.num_domains
    }

    pub fn enumerate(&self) {
        let results = Results::new(self.num_domains);
        let mut threads = Vec::new();

        if self.library.len() == 0 {
            for i in 0..self.num_domains {
                let new_query = self.clone();
                let new_query2 = self.clone();
                let new_results = results.clone();
                let new_results2 = results.clone();
                let handle: thread::JoinHandle<_> = thread::spawn(move || {
                    new_query.enumWithDB(i, new_results);
                    new_query2.enumWithLib(i, new_results2);
                });

                threads.push(handle);
            }
        }
        else {
            for i in 0..self.num_domains {
                let new_query = self.clone();
                let new_results = results.clone();
                let handle: thread::JoinHandle<_> = thread::spawn(move || {
                    new_query.enumWithDB(i, new_results);
                });

                threads.push(handle);
            }
        }

        for child in threads{
            child.join().unwrap();
        }
    }

    fn enumWithDB(&self, domain_position:usize, results: Results) {
        let domain = self.domains[domain_position].clone();
        let store = results.store[domain_position].clone();
        let limit = self.limit.clone();
        enumerator::query_database(domain, store, limit);
    }

    fn enumWithLib(&self, domain_position : usize, results: Results) {
        let domain =  self.domains[domain_position].clone();
        let library = self.library.clone();
        let store = results.store[domain_position].clone();
        library_enumerator::enumerate(domain, library, store);
    }
}
