use std::process::Command;

fn main() {
    let output = Command::new("chroot")
        .arg("testroot")
        .arg("/hello")
        .output()
        .expect("Failed to execute command");

    println!("{:?}", String::from_utf8(output.stdout).unwrap());
}
