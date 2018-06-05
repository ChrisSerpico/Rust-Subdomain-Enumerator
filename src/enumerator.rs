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

/// Takes a domain, a store, and a limit. Queries virustotal.com with the domain, and returns as many subdomains as it finds or as many fit in the limit. 
/// Please note that virustotal, the database we query, will not accept a limit greater than 35. Thus this function will always limit itself to 35 at most. 
/// 
/// # Arguments
/// * 'domain' - A String with the domain you want to enumerate. 
/// * 'store' - An Arc<Mutex<HashSet<String>>> that you want to read the subdomains into. 
/// * 'pool' - A ThreadPool holding threads you want to use for concurrent enumeration. 
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
