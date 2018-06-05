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