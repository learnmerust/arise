mod spawn;

use std::env;
use spawn::new;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
      let ref _path = args[0];
      let ref process = args[1];
      new(process, args[2..].to_vec());
    }
}
