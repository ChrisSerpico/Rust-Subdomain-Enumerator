extern crate reqwest;
extern crate dns_lookup;

use std::sync::Arc;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::mem;
use std::net::IpAddr;
use std::thread;
use self::dns_lookup::lookup_host;
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

    pub fn add_domains(&mut self, domains: Vec<String>){
        self.domains = domains;
        self.num_domains = self.domains.len();
    }

    pub fn add_domain(&mut self, domain: String){
        self.domains.push(domain);
        self.num_domains += 1;
    }

    pub fn set_limit(&mut self, limit: usize){
       	self.limit = limit;
    }

    pub fn set_library(&mut self, library: String){
    	self.library = library;
    }

    pub fn get_num_domains(&self) -> usize{
    	self.num_domains
    }

    pub fn query_database(&self, domain_position:usize, results: Results) {
    	let url = format!("https://www.virustotal.com/ui/domains/{}/subdomains?limit={}", self.domains[domain_position], self.limit);
	    let client = reqwest::Client::new();
	    let virustotal: Resp = client.get(&url).send().unwrap().json().unwrap();

	    for subdomain in virustotal.data.iter(){
	    	results.insert_subdomain(domain_position, subdomain.id.clone())
	    }
    }

 //    pub fn enumerate_library(&self, domain_position:usize, store: Results)  {
	//     let lib_buf;
	//     match File::open(self.library) {
	//         Ok(lib) => {
	//             lib_buf = BufReader::new(lib);
	//         }
	//         Err(error) => {
	//             eprintln!("enumerate: {}\nlibrary enumerator is aborting", error);
	//             return
	//         }
	//     }

	//     // Used to track wildcard records
	//     let mut wildcards : HashSet<IpAddr> = HashSet::new();
	//     self.get_wildcards(&self.domains[domain_position], &mut wildcards);
	//     let wc = Arc::new(wildcards);

	//     // Begin enumeration
	//     let mut prefixes = lib_buf.lines();
	//     while let Some(Ok(prefix)) = prefixes.next() {
	//         let subdomain = format!("{}.{}", prefix, self.domains[domain_position]);
	//         let new_lib = self.library.clone();
	//         let new_wc = wc.clone();
	//         let new_store = store.clone();

	//         thread::spawn(move || {
	//             self.try_subdomain(subdomain, new_lib, new_wc, new_store);
	//         });
	//     }
	// }
	// Takes a string representing the domain name,
	// and a empty hash set for storing wildcard addresses
	// Checks whether the domain name has a wildcard DNS record;
	// if a wildcard record is in use, store its addresses in the hash set
	fn get_wildcards(&self, domain : &String, wc : &mut HashSet<IpAddr>) {
	    // Make up a weird name
	    let name = format!("IfThisWorksThisDomainUsesAWildcardRecord.{}", domain);

	    match lookup_host(name.as_str()) {
	        Ok(vec) => {
	            while let Some(addr) = vec.iter().next() {
	                wc.insert(*addr);
	            }
	        }
	        Err(error) => {
	            eprintln!("get_wildcards: {}", error);
	        }
	    }

	    println!("Discovered wildcard record for host {} with {} IP addresses", domain, wc.len());
	}

	// Takes a string representing the subdomain to try,
	// a string representing the path to the library,
	// a hash set containing wildcard IP addresses,
	// and a hash set for holding discovered subdomains
	// Attempts to resolve subdomain name by querying DNS
	// If successful add subdomain name to the discovered hash set
	// fn try_subdomain(&self, subdomain : String,
	//                  library: String,
	//                  wc : Arc<HashSet<IpAddr>>,
	//                  results : Results) {
	//     if self.query(&subdomain, wc.as_ref()) {
	//         let mut found = results.get_writable_store();
	//         found.insert(subdomain.clone());
	//         mem::drop(found);

	//         // Recurse on valid subdomain
	//         let new = results.clone();
	//         thread::spawn(move || {
	//             self.enumerate_library(subdomain, library, new);
	//         });
	//     }
	// }

	// Takes a string representing the name to query,
	// and a hash set containing wildcard addresses
	// If the name can be resolved and is not a wildcard, return true
	// Otherwise return false
	fn query(&self, name : &String, wc : &HashSet<IpAddr>) -> bool {
	    match lookup_host(name) {
	        Ok(vec) => {
	            let mut iter = vec.iter();
	            while let Some(addr) = iter.next() {
	                if !wc.contains(addr) {
	                    return true
	                }
	            }
	        }
	        Err(error) => {
	            eprintln!("query: {}", error);
	            return false
	        }
	    }
	    return false
	}
        

}
