use std::env;

struct Router {
  networks: Vec<String>,
  routes: Vec<String>,
  updates: Vec<String>,
  relations: Vec<String>,
  sockets: Vec<String>,
  asn: String,
}

impl Router {
  pub fn new(networks: Vec<String>) -> Self {
    Self {
      networks,
      routes : Vec::new(),
      updates : Vec::new(),
      relations : Vec::new(),
      sockets : Vec::new(),
      asn : "127.0.0.1".to_string(),
    }
  }
}

fn main() {
  println!("Hello, world!");
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  let r: Router = Router::new(args);
  //r.run()
}
