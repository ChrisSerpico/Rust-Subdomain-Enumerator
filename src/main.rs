extern crate subdomain_enumerator;
extern crate clap;
use clap::{Arg, App};

// used to read arguments passed on command line
use std::thread;
use subdomain_enumerator::enumerator; 
use std::collections::HashMap;
// use subdomain_enumerator::library_enumerator;


fn main() {
    // read arguments from command line 
    let matches = App::new("Concurrent Subdomain Enumerator")
                          .version("1.0")
                          .about("Queries VirusTotal for subdomains and performs dictionary enumeration.")
                          .arg(Arg::with_name("domains")
                               .short("d")
                               .required(true)
                               .multiple(true)
                               .help("Specifies the domains to enumerate."))
                          .arg(Arg::with_name("limit")
                               .short("l")
                               .help("Specifies the number of subdomains to query for each domain."))
                          .arg(Arg::with_name("wordlist")
                               .short("w")
                               .help("Specifies the wordlist to use for dictionary enumeration."))
                          .get_matches();
    let mut subdomains = HashMap::new();
    let domains: Vec<_> = matches.values_of("domains").unwrap().collect();
    let limit_arg = matches.value_of("limit").unwrap_or("10");
    let limit: usize = limit_arg.parse().unwrap();

    for i in 0..domains.len(){
        thread::spawn(move || {
            enumerator::query_database(&domains[i].to_string(), &mut subdomains, limit);
        });
    }
    
    if matches.is_present("wordlist") {
        let dictionary = matches.value_of("wordlist");
        // library_enumerator::enumerate(domains, dictionary);
    }
}
