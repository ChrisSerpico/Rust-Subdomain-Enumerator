/*!

Vickie Li, Alex Lu Wang, Chris Serpico

Contains code for a subdomain enumerator that finds subdomains
by trying subdomains generated with a library of common words.

ASSUMPTIONS:
    1. The domain name is in the form "domain-name.com",
    so something like "www.domain-name.com" will not work
    2. The library file contains one word per line

*/

extern crate dns_lookup;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::mem;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use self::dns_lookup::lookup_host;

// Takes a string representing the domain to query,
// a string representing the pathname to library,
// a hash map holding domain-subdomains pairs,
// and an optional string representing the top-level domain name;
// the top-level domain name is used for recursive queries
// Appends each subdomain prefix from the library to the domain name
// and passes the concatenated name to resolvers
// If the name is resolvable, it is added as a valid subdomain
// and recursively enumerated on
pub fn enumerate(domain: String,
                 library: String,
                 store : Arc<Mutex<HashMap<String, HashSet<String>>>>,
                 superdomain : Option<String>)  {
    let lib_buf;
    match File::open(&library) {
        Ok(lib) => {
            lib_buf = BufReader::new(lib);
        }
        Err(error) => {
            eprintln!("enumerate: {}\nlibrary enumerator is aborting", error);
            return
        }
    }

    // Used to track wildcard records
    let mut wildcards : HashSet<IpAddr> = HashSet::new();
    get_wildcards(&domain, &mut wildcards);
    let wc = Arc::new(wildcards);

    // Begin enumeration
    let mut prefixes = lib_buf.lines();
    while let Some(Ok(prefix)) = prefixes.next() {
        let subdomain = format!("{}.{}", prefix, domain);
        let new_lib = library.clone();
        let new_wc = wc.clone();
        let new_store = store.clone();
        let top;
        if let Some(sd) = superdomain.clone() {
            top = sd;
        } else {
            top = domain.clone();
        }

        thread::spawn(move || {
            try_subdomain(subdomain, new_lib, new_wc, new_store, top);
        });
    }
}

// Takes a string representing the domain name,
// and a empty hash set for storing wildcard addresses
// Checks whether the domain name has a wildcard DNS record;
// if a wildcard record is in use, store its addresses in the hash set
fn get_wildcards(domain : &String, wc : &mut HashSet<IpAddr>) {
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
// a hash map holding domain-subdomains pairs,
// and a string indicating the top-level domain of the current subdomain
// Attempts to resolve subdomain name by querying DNS
// If successful add subdomain name to the hash map
fn try_subdomain(subdomain : String,
                 library: String,
                 wc : Arc<HashSet<IpAddr>>,
                 store : Arc<Mutex<HashMap<String, HashSet<String>>>>,
                 domain : String) {
    if query(&subdomain, wc.as_ref()) {
        let mut map = store.lock().unwrap();

        if !map.contains_key(domain.as_str()) {
            map.insert(domain.clone(), HashSet::new());
        }
        map.get_mut(&domain).unwrap().insert(subdomain.clone());

        mem::drop(map);

        // Recurse on valid subdomain
        let new = store.clone();
        thread::spawn(move || {
            enumerate(subdomain, library, new, Some(domain));
        });
    }
}

// Takes a string representing the name to query,
// and a hash set containing wildcard addresses
// If the name can be resolved and is not a wildcard, return true
// Otherwise return false
fn query(name : &String, wc : &HashSet<IpAddr>) -> bool {
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

#[cfg(test)]
mod query_tests {
    use super::*;

    #[test]
    fn localhost_test() {
        let name = String::from("localhost");
        let wc = HashSet::new();

        assert_eq!(query(&name, &wc), true);
    }

    #[test]
    fn localhost_fail_test() {
        let name = String::from("localhost");
        let mut wc : HashSet<IpAddr> = HashSet::new();

        match lookup_host(&name) {
            Ok(vec) => {
                let mut iter = vec.iter();
                while let Some(addr) = iter.next() {
                    wc.insert(*addr);
                }
            }
            Err(_e) => {}
        }

        assert_eq!(query(&name, &wc), false);
    }
}

#[cfg(test)]
mod get_wildcards_tests {
    use super::*;

    // not sure how to test case with wildcard

    #[test]
    fn no_wildcard_test() {
        let name = String::from("localhost");
        let mut wc = HashSet::new();
        get_wildcards(&name, &mut wc);

        assert_eq!(wc.len(), 0);
    }
}
