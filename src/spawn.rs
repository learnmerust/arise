use std::process::Command;

pub fn new(process: &str, args: Vec<String>) {
  let _args = args.clone();
  let mut child = Command::new(process)
    .args(args)
    .spawn()
    .unwrap();

  let restart = || new(process, _args);

  match child.try_wait() {
    Ok(Some(status)) => println!("exited with: {}", status),
    Ok(None) => {
      // status not ready yet, let's wait
      let res = child.wait();
      println!("Exited with status code: {:?}", res.unwrap().code());
      println!("arise!");
      restart()
    }
    Err(error) => {
      println!("error attempting to wait: {}", error);
      restart()
    }
  }
}
