//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::process::Command;
use std::thread;
use std::time::Duration;

#[test]
fn add_update_delete() {
    let mut cargo_run = Command::new("cargo");
    if let Ok(mut child) = cargo_run.arg("run").spawn() {
        thread::sleep(Duration::from_secs(1)); //time to start server

        let curl_output = Command::new("curl")
            .args(&[
                "-X",
                "POST",
                "-H",
                "Content-Type: text/plain",
                "--data-binary",
                "newword\n3000\nновое слово",
                "http://localhost/words",
            ])
            .output()
            .expect("Failed to execute command");
        let found = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");
        assert_eq!("Word \"newword\" is added \u{1F60E}", found);

        let curl_output = Command::new("curl")
            .args(&[
                "-X",
                "POST",
                "-H",
                "Content-Type: text/plain",
                "--data-binary",
                "newword\n3000\nновое слово",
                "http://localhost/words",
            ])
            .output()
            .expect("Failed to execute command");
        let found = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");
        assert_eq!("Word \"newword\" is already exist \u{1F60E}", found);

        child.kill().expect("Failed to stop cargo");
    } else {
        assert!(false);
    }
}
