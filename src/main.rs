extern crate subdomain_enumerator;
extern crate clap;

use clap::{Arg, App};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use subdomain_enumerator::query;
use subdomain_enumerator::results;



fn main() {
    let mut config = Config::new();
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

    config.add_domains(matches.values_of_lossy("domains").unwrap());
    let results = Results::new(config.get_num_domains());

    if matches.is_present("limit") {
        let limit_arg = matches.value_of("limit").unwrap();
        config.set_limit(limit_arg.parse().unwrap());
    }  

    let mut threads = Vec::new();

    if matches.is_present("wordlist") {
        config.set_library(matches.value_of("wordlist").unwrap().to_string());

        for i in 0..config.get_num_domains() {         
            let handle: thread::JoinHandle<_> = thread::spawn(move || {
                query.query_database(i, results.get_store());
                query.enumerate_library(i, results.get_store());
            });

            threads.push(handle);
        }
    }
    else {
        for i in 0..config.get_num_domains() {
            let handle: thread::JoinHandle<_> = thread::spawn(move || {
                query.query_database(i, results.get_store());
            });

            threads.push(handle);
        }  
    }

    for child in threads{
        child.join().unwrap();
    }
}
