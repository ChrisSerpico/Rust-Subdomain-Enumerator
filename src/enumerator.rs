// Contains code for a subdomain enumerator that finds subdomains by querying public datasets

extern crate reqwest;
use std::collections::HashSet;

#[derive(Deserialize, Debug)]
struct Resp {
    data: Vec<Subdomain>,
}

#[derive(Deserialize, Debug)]
struct Subdomain {
    id: String,
}

// takes a domain name as a string and returns a vector of subdomains as strings 
pub fn enumerate(domain: &String) -> HashSet<String> {
	let limit = 10;
    let mut subdomains = HashSet::new();
    let url = format!("https://www.virustotal.com/ui/domains/{}/subdomains?limit={}", domain, limit);
    let client = reqwest::Client::new();
    let virustotal: Resp = client.get(&url).send().unwrap().json().unwrap();
    // println!("{}\n{:?}", url, virustotal);
    for subdomain in virustotal.data.iter(){
    	subdomains.insert(subdomain.id.clone());
    }
    println!("{:?}", subdomains);
    subdomains
}