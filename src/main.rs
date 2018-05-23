extern crate subdomain_enumerator;

// used to read arguments passed on command line
use std::env; 
use subdomain_enumerator::enumerator; 
use subdomain_enumerator::library_enumerator; 

use std::process;
fn main() {
    // read arguments from command line 
    let args: Vec<String> = env::args().collect(); 

    // the first argument is necessary, and gives the domain we want to find subdomains of 
    if args.len() < 2 {
        println!("At least one argument specifying the domain to enumerate is required."); 
        process::exit(1); 
    }
    else if args.len() < 3 {
        // Case where we don't use dictionary enumeration 
        println!("Enumeration will be performed using only database querying."); 
        let domain = &args[1]; 
        enumerator::enumerate(); 
    }
    else {
        // Case where we do use dictionary enumeration 
        println!("Enumeration will be performed using both database querying and library enumeration."); 
        let domain = &args[1]; 
        let dictionary = &args[2]; 
        enumerator::enumerate(); 
        library_enumerator::enumerate(); 
    }
}
