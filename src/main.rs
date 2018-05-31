extern crate subdomain_enumerator;
extern crate clap;

use clap::{Arg, App};
// used to read arguments passed on command line

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use subdomain_enumerator::enumerator;
use subdomain_enumerator::library_enumerator;

fn main() {
    // read arguments from command line 
    let matches = App::new("Concurrent Subdomain Enumerator")
                          .version("1.0")
                          .about("Queries VirusTotal for subdomains and performs dictionary enumeration.")
                          .arg(Arg::with_name("domains")
                               .required(true)
                               .takes_value(true)
                               .multiple(true)
                               .help("Specifies the domains to enumerate."))
                          .arg(Arg::with_name("limit")
                               .short("l")
                               .help("Specifies the number of subdomains to query for each domain."))
                          .arg(Arg::with_name("wordlist")
                               .short("w")
                               .takes_value(true)
                               .help("Specifies the wordlist to use for dictionary enumeration."))
                          .get_matches();

    let domains: Vec<_> = matches.values_of("domains").unwrap().collect();
    let subdomains : Vec<Arc<Mutex<HashSet<String>>>>  = vec![Arc::new(Mutex::new(HashSet::new())); domains.len()];
    let limit_arg = matches.value_of("limit").unwrap_or("10");
    let limit: usize = limit_arg.parse().unwrap();
    let lim = limit.clone();
    let mut threads = Vec::new();

    if matches.is_present("wordlist") {
        let dictionary = matches.value_of("wordlist").unwrap();

        for i in 0..domains.len() {
            let domain = domains[i].to_string();
            let library = dictionary.to_string();
            let store = subdomains[i].clone();           
            let handle: thread::JoinHandle<_> = thread::spawn(move || {
                enumerator::query_database(domain.clone(), store.clone(), lim);
                library_enumerator::enumerate(domain, library, store);
            });

            threads.push(handle);
        }
    }
    else {
        for i in 0..domains.len() {
            let domain = domains[i].to_string();
            let store = subdomains[i].clone();
            let handle: thread::JoinHandle<_> = thread::spawn(move || {
                enumerator::query_database(domain.clone(), store.clone(), lim);
            });

            threads.push(handle);
        }  
    }

    for child in threads{
        child.join().unwrap();
    }
}
