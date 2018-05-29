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

use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::net::IpAddr;
use self::dns_lookup::lookup_host;

// Takes a string representing the domain to query,
// a string representing the pathname to library,
// and an optional string representing the top-level domain name;
// the top-level domain name is only given for recursive queries
// Appends each subdomain name from the library to the domain name
// and attempts to get an IP address by querying the DNS server
// If a valid IP address is obtained it is stored in the appropriate buffer
pub fn enumerate(domain: &String, library: &String, superdomain : Option<&String>)  {
    let lib_buf;
    match File::open(library) {
        Ok(lib) => {
            lib_buf = BufReader::new(lib);
        }
        Err(_e) => {
            eprintln!("Error opening library");
            return
        }
    }

    // Used to track wildcard records
    let mut wc : HashSet<IpAddr> = HashSet::new();
    get_wildcards(domain, &mut wc);

    // Begin enumeration
    // Should spawn a thread
    let mut subdomains = lib_buf.lines();
    while let Some(Ok(subdomain)) = subdomains.next() {
        let name = subdomain + domain;
        if let Some(vec) = query(&name, &mut wc) {
            if !vec.is_empty() {
                if let Some(sd) = superdomain {
                    // lock buffer
                    // if domain exists, we do nothing
                }
                else {

                }

                // Recurse on valid subdomain
                // Should spawn a thread
                enumerate(&name, library, Some(domain));
            }
        }
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
        Err(_e) => {
            eprintln!("Warning: failed to query wildcard record for host {}", domain);
        }
    }

    println!("Discovered wildcard record for host {} with {} IP addresses", domain, wc.len());
}

// Takes a string representing the name to query,
// and a hash set containing wildcard addresses
// If the name can be resolved and is not a wildcard,
// returns query results as a vector of IP addresses
// Returns none otherwise
fn query(name : &String, wc : &mut HashSet<IpAddr>) -> Option<Vec<IpAddr>> {
    let mut addresses : Vec<IpAddr> = Vec::new();

    match lookup_host(name) {
        Ok(vec) => {
            while let Some(addr) = vec.iter().next() {
                if !wc.contains(addr) {
                    addresses.push(*addr)
                }
            }
        }
        Err(_e) => {
            return None
        }
    }

    println!("Discovered {} IP addresses for subdomain {}", addresses.len(), name);
    Some(addresses)
}

// TODO write tests