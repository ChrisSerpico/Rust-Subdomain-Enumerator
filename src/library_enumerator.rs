/*!

Vickie Li, Alex Lu Wang, Chris Serpico

Contains code for a subdomain enumerator that finds subdomains
by trying subdomains from a library of common subdomains.

ASSUMPTIONS:
    1. The domain name is in the form domain-name.com;
    something like www.domain-name.com will not work
    2. The library file contains one subdomain name per line

*/

extern crate trust_dns_resolver;

use std::collections::HashSet;
use std::io::{BufReader, BufRead, Write};
use std::fs::{File, OpenOptions};
use std::net::*;
use self::trust_dns_resolver::{Resolver, config::*};

// Takes a string representing the domain to query,
// and a string representing the pathname to library
// Appends each subdomain name from the library to the domain name
// and attempts to get an IP address by querying the DNS server
// If a valid IP address is obtained, writes the subdomain name and IP address to file
pub fn enumerate(domain: &String, library: &String)  {
    let lib = File::open(library).expect("Error: opening library"); // TODO make this not panic
    let lib_buf = BufReader::new(lib);

    let mut output = OpenOptions::new()
        .create(true)
        .append(true)
        .open(domain+"_subdomains_list")
        .expect("Error: opening output file");

    // Used to track wildcard records
    let mut wc : HashSet<IpAddr> = HashSet::new();
    get_wildcards(domain, &mut wc);

    // Begin enumeration
    // This is currently sequential
    let mut subdomains = lib_buf.lines();
    while let Some(subdomain) = subdomains.next().expect("Error: reading from library") {
        let name = subdomain + domain;
        if let Some(result) = query(name, &mut wc) {
            output.write_all(result.as_bytes());
        }
    }
}


// Takes a string representing the domain name,
// and a empty hash set for storing wildcard addresses
// Checks whether the domain name has wildcard DNS records;
// if wildcard records are in use, store their addresses in the hash set
fn get_wildcards(domain : &String, wc : &mut HashSet<IpAddr>) {
    // Make up a weird name
    let name = "asdfjkl;1423" + domain;

    let mut resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let mut response = resolver.lookup_ip(name).unwrap().iter();

    while let Some(addr) = response.next().expect("Error: iterating through resolver") {
        wc.insert(addr);
    }
}

// Takes a string representing the name to query,
// and a hash set containing wildcard addresses
// If the name can be resolved and is not a wildcard,
// returns query results as string
// Returns none otherwise
fn query(name : String, wildcards : &mut HashSet<IpAddr>) -> Option<String> {
    let mut resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let mut response = resolver.lookup_ip(name.as_str()).unwrap().iter();

    if response.count() != 0 {
        let mut result = String::from("subdomain name: {}\n IP address:\n");
        while let Some(addr) = response.next.expect("Error: iterating through resolver") {
            if !wildcards.contains(addr) {
                result.push_str(format!("{}", addr).as_str());
            }
        }
        return Some(result)
    }
    else {
        return None
    }
}

// TODO write tests