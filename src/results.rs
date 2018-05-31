#[derive(Debug)]
pub struct Results {
    store: Vec<Arc<Mutex<HashSet<String>>>>,
}

impl Query {
    pub fn new(num_domains: usize) -> Self {
        Results{
        	store: vec![Arc::new(Mutex::new(HashSet::new())); num_domains];,
        }
    }
}