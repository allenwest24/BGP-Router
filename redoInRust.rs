// Imports used.
use std::env;
use std::collections::HashMap;

// The Router class used to simulate a BGP router.
struct Router {
  networks: HashMap<String, String>,
  routes: Vec<String>,
  updates: Vec<String>,
  relations: Vec<String>,
  sockets: Vec<String>,
  asn: String,
}

// Methods used to implement the functionality of our Router.
impl Router {
  pub fn new(networks: HashMap<String, String>, asn: String) -> Self {
    Self {
      networks,
      routes : Vec::new(),
      updates : Vec::new(),
      relations : Vec::new(),
      sockets : Vec::new(),
      asn,
    }
  }
}

// The main method for this router program.
fn main() {
  // Parse the arguments.
  let mut args: Vec<String> = env::args().collect();
  // The asn is the first arguments after the program call.
  let mut asn : String = args.remove(1);
  // The rest of the arguments should be relationships.
  let mut networks = HashMap::new();
  for ii in 1..args.len() {
    networks.insert(args.get(ii).unwrap().into(), (ii + 1).to_string());
  }
  
  // Initialize our router.
  let r: Router = Router::new(networks, asn);
}
