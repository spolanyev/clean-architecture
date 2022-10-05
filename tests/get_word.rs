//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::process::Command;
use std::thread;
use std::time::Duration;

#[test]
fn get() {
    let mut cargo_run = Command::new("cargo");
    if let Ok(mut child) = cargo_run.arg("run").spawn() {
        thread::sleep(Duration::from_secs(1)); //time to start server

        let curl_output = Command::new("curl")
            .arg("http://localhost/words/ability")
            .output()
            .expect("Failed to execute command");
        let found = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        let curl_output = Command::new("curl")
            .arg("http://localhost/words/qazxsw")
            .output()
            .expect("Failed to execute command");
        let not_found = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        child.kill().expect("Failed to stop cargo");

        assert!(found.starts_with("Word \"ability\" is found"));
        assert!(not_found.starts_with("Word \"qazxsw\" is not found"));
    } else {
        assert!(false);
    }
}
