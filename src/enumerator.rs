// Contains code for a subdomain enumerator that finds subdomains by querying public datasets

extern crate reqwest;
use std::collections::{HashSet, HashMap};
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
pub fn query_database(domain: String, store: Arc<Mutex<HashMap<String, HashSet<String>>>>, limit: usize){
    let mut map = store.lock().unwrap();

    if !map.contains_key(domain.as_str()) {
        map.insert(domain.clone(), HashSet::new());
    }

    let url = format!("https://www.virustotal.com/ui/domains/{}/subdomains?limit={}", domain, limit);
    let client = reqwest::Client::new();
    let virustotal: Resp = client.get(&url).send().unwrap().json().unwrap();
    for subdomain in virustotal.data.iter(){
        map.get_mut(&domain).unwrap().insert(subdomain.id.clone());
    }
    
}