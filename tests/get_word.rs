//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::process::Command;
use std::thread;
use std::time::Duration;

#[test]
fn get() {
    let mut cargo = Command::new("cargo");
    if let Ok(mut child) = cargo.arg("run").spawn() {
        thread::sleep(Duration::from_secs(1)); //time to start server

        let curl_output = Command::new("curl")
            .arg("http://localhost/words/testworda")
            .output()
            .expect("Failed to execute command");
        let found = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        let curl_output = Command::new("curl")
            .arg("http://localhost/words/notexistant")
            .output()
            .expect("Failed to execute command");
        let not_found = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        let curl_output = Command::new("curl")
            .arg("-I")
            .arg("http://localhost/words/testworda")
            .output()
            .expect("Failed to execute command");
        let bad_request = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        child.kill().expect("Failed to stop cargo");

        assert!(found.starts_with("testworda<br \\>\n1000<br \\>\nлебедь"));
        assert!(not_found.starts_with("Word \"notexistant\" is not found"));
        assert!(bad_request.starts_with("HTTP/1.1 400 Bad Request"));
    } else {
        assert!(false);
    }
}
