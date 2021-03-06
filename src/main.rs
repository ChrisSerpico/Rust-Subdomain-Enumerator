extern crate subdomain_enumerator;
extern crate clap;

use clap::{Arg, App};
use subdomain_enumerator::query::Query;

fn main() {
    let mut query = Query::new();
    let matches = App::new("Concurrent Subdomain Enumerator")
                          .version("1.0")
                          .about("Queries VirusTotal for subdomains and performs dictionary enumeration.")
                          .arg(Arg::with_name("domains")
                               .short("d")
                               .required(true)
                               .takes_value(true)
                               .multiple(true)
                               .help("Specifies the domains to enumerate."))
                          .arg(Arg::with_name("limit")
                               .short("l")
                               .takes_value(true)
                               .help("Specifies the number of subdomains to query for each domain."))
                          .arg(Arg::with_name("wordlist")
                               .short("w")
                               .takes_value(true)
                               .help("Specifies the wordlist to use for dictionary enumeration."))
                          .get_matches();
    query.add_domains(matches.values_of_lossy("domains").unwrap());

    if matches.is_present("limit") {
        let limit_arg = matches.value_of("limit").unwrap();
        query.set_limit(limit_arg.parse().unwrap());
    }  

    if matches.is_present("wordlist") {
        query.set_library(matches.value_of("wordlist").unwrap().to_string());
    }

    let subdomains = query.enumerate();
    subdomains.print_subdomains();

}