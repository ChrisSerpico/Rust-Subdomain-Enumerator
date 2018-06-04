/*!
Vickie Li, Alex Lu Wang, Chris Serpico
Contains code for a subdomain enumerator that finds subdomains
by trying subdomains generated with a library of common words.
ASSUMPTIONS:
    1. The domain name is in the form "domain-name.com",
    so something like "www.domain-name.com" will not work
    2. The library file contains one word per line
*/

extern crate chan;
extern crate dns_lookup;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::mem;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use self::chan::WaitGroup;
use self::dns_lookup::lookup_host;

/// Takes a domain, a library, a store, and a wg (WaitGroup). For each word supplied in library, checkes to see whether the word specifies a subdomain of domain. If it does, the found subdomain is added to store. 
///
/// # Examples
/// 
pub fn enumerate(domain: String,
                 library: String,
                 store : Arc<Mutex<HashSet<String>>>,
                 wg : WaitGroup) {
    let lib_buf;
    match File::open(&library) {
        Ok(lib) => {
            lib_buf = BufReader::new(lib);
        }
        Err(error) => {
            // TODO should propagate error instead
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
        let new_wg = wg.clone();
        let new_wg2 = wg.clone();

        wg.add(1);
        thread::spawn(move || {
            try_subdomain(subdomain, new_lib, new_wc, new_store, new_wg);
            new_wg2.done();
        });
    }
}

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
            // TODO should propagate error instead
            eprintln!("get_wildcards: {}", error);
        }
    }
}

fn try_subdomain(subdomain : String,
                 library: String,
                 wc : Arc<HashSet<IpAddr>>,
                 store : Arc<Mutex<HashSet<String>>>,
                 wg : WaitGroup) {
    if query(&subdomain, wc.as_ref()) {
        let mut found = store.lock().unwrap();
        found.insert(subdomain.clone());
        mem::drop(found);

        // Recurse on valid subdomain
        let new_store = store.clone();
        let new_wg = wg.clone();

        wg.add(1);
        thread::spawn(move || {
            enumerate(subdomain, library, new_store, new_wg);
            wg.done();
        });
    }
}

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
            eprintln!("query: {} \n target: {}\n", error, name);
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
