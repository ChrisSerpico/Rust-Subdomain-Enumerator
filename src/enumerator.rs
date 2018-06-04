//! A subdomain enumerator. Finds subdomains of a given subdomain by querying a public dataset (virustotal.com). 

extern crate chan;
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

/// Takes a domain, a store, and a limit. Queries virustotal.com with the domain, and adds found subdomains to store until the number of subdomains found reaches the amount specified by limit.  
pub fn query_database(domain: String,
                      store: Arc<Mutex<HashSet<String>>>,
                      limit: usize) {
    let url = format!("https://www.virustotal.com/ui/domains/{}/subdomains?limit={}", domain, limit);
    let client = reqwest::Client::new();
    let virustotal: Resp = client.get(&url).send().unwrap().json().unwrap();

    let mut set = store.lock().unwrap();
    for subdomain in virustotal.data.iter(){
        set.insert(subdomain.id.clone());
    }
}
