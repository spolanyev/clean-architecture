//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::process::Command;
use std::thread;
use std::time::Duration;

#[test]
fn get_health_page() {
    //set env=dev
    let mut cargo = Command::new("cargo");
    if let Ok(mut child) = cargo.arg("run").env("RUST_ENV", "development").spawn() {
        thread::sleep(Duration::from_secs(1)); //time to start server

        let curl_output = Command::new("curl")
            .arg("http://localhost/health")
            .output()
            .expect("Failed to execute command");
        let page = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        child.kill().expect("Failed to stop cargo");

        assert_eq!("200 OK\nRUST_ENV=development\n", page);
    } else {
        assert!(false);
    }
    //set env=prod
    let mut cargo = Command::new("cargo");
    if let Ok(mut child) = cargo.arg("run").env("RUST_ENV", "production").spawn() {
        thread::sleep(Duration::from_secs(1)); //time to start server

        let curl_output = Command::new("curl")
            .arg("http://localhost/health")
            .output()
            .expect("Failed to execute command");
        let page = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        child.kill().expect("Failed to stop cargo");

        assert_eq!("200 OK\nRUST_ENV=production\n", page);
    } else {
        assert!(false);
    }
}
