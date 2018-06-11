//! A concurrent subdomain enumerator that utilizes both passive and active subdomain enumeration techniques. 
//! It will help penetration testers and bug bounty hunters gather subdomain information for the domain that they are targeting. 
//! This tool is capable of performing subdomain enumeration in two ways: 
//! 1. It queres VirusTotal for subdomain information.
//! 2. It also performs a dictionary based enumeration based on a user-defined word list. 
//! # Examples
//!
//! Performing subdomain enumeration on a query with no library, and therefore no library enumeration. 
//! ```
//! let mut q = Query::new(); 
//! q.add_domains(vec!["facebook.com", "google.com"]; 
//! let subdomains = q.enumerate(); 
//! subdomains.print_subdomains(); 
//! ```
//! 
//! Performing subdomain enumeration on a query with a library added. This means that library enumeration will be performed along with normal database querying.
//! Note that a library is passed as a string holding a path to an external file. 
//! ```
//! let mut q = Query::new();
//! q.add_domains(vec!["facebook.com", "google.com"]; 
//! q.set_library("path_to_library.txt"); 
//! let subdomains = q.enumerate(); 
//! subdomains.print_subdomains(); 
//! 
#[macro_use]
extern crate serde_derive;

pub mod enumerator; 
pub mod library_enumerator;
pub mod query;
pub mod results;