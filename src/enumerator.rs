//! A subdomain enumerator. Finds subdomains of a given subdomain by querying a public dataset (virustotal.com). 

extern crate reqwest;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Debug)]
struct Resp {
    data: Vec<Subdomain>,
}

#[derive(Deserialize, Debug)]
struct Subdomain {
    id: String,
}

// takes a domain name as a string and returns a vector of subdomains as strings 
pub fn query_database(domain: String,
                      store: Arc<Mutex<HashSet<String>>>,
                      limit: usize) {

    // small query size makes virustotal happy
    let mut new_limit = limit;
    if new_limit > 35 {new_limit = 35;}

    let url = format!("https://www.virustotal.com/ui/domains/{}/subdomains?limit={}", domain, new_limit);
    let client = reqwest::Client::new();
    let virustotal: Resp = client.get(&url).send().unwrap().json().unwrap();

    let mut set = store.lock().unwrap();
    for subdomain in virustotal.data.iter(){
        set.insert(subdomain.id.clone());
    }
}
