extern crate reqwest;
extern crate chan;

use std::thread;
use enumerator;
use library_enumerator;
use results::Results;
use self::chan::WaitGroup;

#[derive(Deserialize, Debug)]
struct Resp {
    data: Vec<Subdomain>,
}

#[derive(Deserialize, Debug)]
struct Subdomain {
    id: String,
}


#[derive(Debug, Clone)]
pub struct Query {
    domains: Vec<String>,
    library: String,
    limit: usize,
    num_domains: usize,

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

    pub fn add_domain(&mut self, domain: String){
        self.domains.push(domain);
        self.num_domains += 1;
    }

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

    pub fn get_num_domains(&self) -> usize{
    	self.num_domains
    }

    pub fn enumerate(&self) {
        let results = Results::new(self.num_domains);
        let wg = WaitGroup::new();

        if self.library.len() != 0 {
            for i in 0..self.num_domains {
                let new_query = self.clone();
                let new_query2 = self.clone();
                let new_results = results.clone();
                let new_results2 = results.clone();
                let new_wg = wg.clone();
                let new_wg2 = wg.clone();

                wg.add(1);
                thread::spawn(move || {
                    new_query.enum_with_db(i, new_results);
                    new_query2.enum_with_lib(i, new_results2, new_wg);
                    new_wg2.done();
                });
            }
        }
        else {
            for i in 0..self.num_domains {
                let new_query = self.clone();
                let new_results = results.clone();
                let new_wg = wg.clone();

                wg.add(1);
                thread::spawn(move || {
                    new_query.enum_with_db(i, new_results);
                    new_wg.done();
                });
            }
        }

        wg.wait();
    }

    fn enum_with_db(&self, domain_position:usize, results: Results) {
        let domain = self.domains[domain_position].clone();
        let store = results.store[domain_position].clone();
        let limit = self.limit.clone();
        enumerator::query_database(domain, store, limit);
    }

    fn enum_with_lib(&self, domain_position : usize, results: Results, wg : WaitGroup) {
        let domain =  self.domains[domain_position].clone();
        let library = self.library.clone();
        let store = results.store[domain_position].clone();
        library_enumerator::enumerate(domain, library, store, wg);
    }
}
