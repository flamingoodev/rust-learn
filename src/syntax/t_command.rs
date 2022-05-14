use std::process::{Command, Output};

#[test]
fn t_command_test() {
    let child = Command::new("ls")
        .arg("-al")
        .spawn()
        .expect("ls command failed to start");
    match child.wait_with_output() {
        Ok(out) => {
            println!("{:?}", out);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
