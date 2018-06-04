extern crate chan;
extern crate reqwest;

use std::thread;
use enumerator;
use library_enumerator;
use results::Results;
use self::chan::WaitGroup;

#[derive(Deserialize, Debug)]
struct Resp {
    data:   Vec<Subdomain>,
}

#[derive(Deserialize, Debug)]
struct Subdomain {
    id: String,
}

/// Represents a user supplied query, where domains is the list of domains that will have their subdomains enumerated and library is a wordlist supplied for library enumeration.  
#[derive(Debug, Clone)]
pub struct Query {
    domains:        Vec<String>,
    library:        String,
    limit:          usize,
    num_domains:    usize,
}

impl Query {
    pub fn new() -> Self {
        Query {
            domains: Vec::new(),
            library: String::new(),
            limit: 10,
            num_domains: 0,
        }
    }
    
    /// Add a domain (as a string) to be enumerated. 
    pub fn add_domain(&mut self, domain: String){
        self.domains.push(domain);
        self.num_domains += 1;
    }
    
    /// Add multiple domains (as a vector of strings) to be enumerated. 
    pub fn add_domains(&mut self, domains: Vec<String>){
        self.domains = domains;
        self.num_domains = self.domains.len();
    }
   
    pub fn set_library(&mut self, library: String){
        self.library = library;
    }

    pub fn set_limit(&mut self, limit: usize){
        self.limit = limit;
    }
    
    /// Performs subdomain enumeration on all domains held in the domains variable. The found subdomains will be returned collectively in a single Results object. If a library is supplied, then both library enumeration and a database query will be performed. If no library is supplied, then only a database query will be performed. 
    /// 
    /// # Examples
    ///
    /// Performing subdomain enumeration on a query with no library, and therefore no library enumeration. 
    /// '''
    /// let mut q = Query::new(); 
    /// q.add_domains(vec!["facebook.com", "google.com"]; 
    /// let subdomains = q.enumerate(); 
    /// subdomains.print_subdomains(); 
    /// ''' 
    /// 
    /// Performing subdomain enumeration on a query with a library added. This means that library enumeration will be performed along with normal database querying.
    /// Note that a library is passed as a string holding a path to an external file. 
    /// '''
    /// let mut q = Query::new();
    /// q.add_domains(vec!["facebook.com", "google.com"]; 
    /// q.set_library("path_to_library.txt"); 
    /// let subdomains = q.enumerate(); 
    /// subdomains.print_subdomains(); 
    /// ''' 
    pub fn enumerate(&self) -> Results{
        let results = Results::new(self.num_domains, self.domains.clone());
        let wg = WaitGroup::new();

        if self.library.len() != 0 {
            for i in 0..self.num_domains {
                // args for query_database
                let db_arg1 = self.domains[i].clone();
                let db_arg2 = results.store[i].clone();
                let db_arg3 = self.limit.clone();

                // args for library enum
                let lib_arg1 =  self.domains[i].clone();
                let lib_arg2 = self.library.clone();
                let lib_arg3 = results.store[i].clone();
                let lib_arg4 = wg.clone();
                let new_wg = wg.clone();

                wg.add(1);
                thread::spawn(move || {
                    enumerator::query_database(db_arg1, db_arg2, db_arg3);
                    library_enumerator::enumerate(lib_arg1, lib_arg2, lib_arg3, lib_arg4);
                    new_wg.done();
                });
            }
        }
        else {
            for i in 0..self.num_domains {
                let db_arg1 = self.domains[i].clone();
                let db_arg2 = results.store[i].clone();
                let db_arg3 = self.limit.clone();
                let new_wg = wg.clone();

                wg.add(1);
                thread::spawn(move || {
                    enumerator::query_database(db_arg1, db_arg2, db_arg3);
                    new_wg.done();
                });
            }
        }

        wg.wait();

        // TODO do something with results
        results
    }
}
