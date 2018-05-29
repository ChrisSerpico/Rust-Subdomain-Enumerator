/*!

Vickie Li, Alex Lu Wang, Chris Serpico

Contains code for a subdomain enumerator that finds subdomains
by trying subdomains from a library of common subdomains.

ASSUMPTIONS:
    1. The domain name is in the form domain-name.com;
    something like www.domain-name.com will not work
    2. The library file contains one subdomain name per line

*/

extern crate dns_lookup;

use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::mem;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use self::dns_lookup::lookup_host;

// Takes a string representing the domain to query,
// a string representing the pathname to library,
// and an optional string representing the top-level domain name;
// the top-level domain name is only given for recursive queries
// Appends each subdomain name from the library to the domain name
// and attempts resolve the name by querying the DNS server
// If the name is resolvable, it is added as a valid subdomain
pub fn enumerate(domain: String,
                 library: String,
                 store : Arc<Mutex<HashMap<String, HashSet<String>>>>,
                 superdomain : Option<String>)  {
    let lib_buf;
    match File::open(&library) {
        Ok(lib) => {
            lib_buf = BufReader::new(lib);
        }
        Err(_e) => {
            eprintln!("Error opening library, library enumerator exiting");
            return
        }
    }

    // Used to track wildcard records
    let mut wildcards : HashSet<IpAddr> = HashSet::new();
    get_wildcards(&domain, &mut wildcards);
    let wc = Arc::new(wildcards);

    // Begin enumeration
    let mut subdomains = lib_buf.lines();
    while let Some(Ok(subdomain)) = subdomains.next() {
        let arg1 = format!("{}.{}", subdomain, domain);
        let arg2 = library.clone();
        let arg3 = wc.clone();
        let arg4 = store.clone();
        let arg5;
        if let Some(sd) = superdomain.clone() {
            arg5 = sd;
        } else {
            arg5 = domain.clone();
        }

        thread::spawn(move || {
            enumerate_branch(arg1, arg2, arg3, arg4, arg5);
        });
    }
}

// Takes a string representing the domain name,
// and a empty hash set for storing wildcard addresses
// Checks whether the domain name has a wildcard DNS record;
// if a wildcard record is in use, store its addresses in the hash set
fn get_wildcards(domain : &String, wc : &mut HashSet<IpAddr>) {
    // Make up a weird name
    let name = format!("asdfjklv1423.{}", domain);

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

fn enumerate_branch(subdomain : String,
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
            while let Some(addr) = vec.iter().next() {
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

// TODO write tests