// Contains code for a subdomain enumerator that finds subdomains by querying public datasets

extern crate reqwest;
use std::collections::{HashSet, HashMap};

#[derive(Deserialize, Debug)]
struct Resp {
    data: Vec<Subdomain>,
}

#[derive(Deserialize, Debug)]
struct Subdomain {
    id: String,
}

// takes a domain name as a string and returns a vector of subdomains as strings 
pub fn query_database(domain: &String, results: &mut HashMap<String, HashSet<String>>, limit: usize){
    let mut current_domain = HashSet::new();
    let url = format!("https://www.virustotal.com/ui/domains/{}/subdomains?limit={}", domain, limit);
    let client = reqwest::Client::new();
    let virustotal: Resp = client.get(&url).send().unwrap().json().unwrap();
    for subdomain in virustotal.data.iter(){
        current_domain.insert(subdomain.id.clone());
    }
    results.insert(domain.to_string(), current_domain);
}