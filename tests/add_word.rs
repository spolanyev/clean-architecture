//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::process::Command;
use std::thread;
use std::time::Duration;

#[test]
fn add_update_delete() {
    let mut cargo_run = Command::new("cargo");
    if let Ok(mut child) = cargo_run.arg("run").spawn() {
        thread::sleep(Duration::from_secs(1)); //time to start server

        //sure not exist
        let curl_output = Command::new("curl")
            .arg("http://localhost/words/newword")
            .output()
            .expect("Failed to execute command");
        let page_not_exist = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        //add new word
        let curl_output = Command::new("curl")
            .args(&[
                "-X",
                "POST",
                "http://localhost/words",
                "-H",
                "Content-Type: text/plain",
                "--data-binary",
                "newword\n5000\nновое слово\n",
            ])
            .output()
            .expect("Failed to execute command");
        let page_add_new = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        //add existing word
        let curl_output = Command::new("curl")
            .args(&[
                "-X",
                "POST",
                "http://localhost/words",
                "-H",
                "Content-Type: text/plain",
                "--data-binary",
                "newword\n5000\nновое слово\n",
            ])
            .output()
            .expect("Failed to execute command");
        let page_add_existing = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        //update word
        let curl_output = Command::new("curl")
            .args(&[
                "-X",
                "PUT",
                "http://localhost/words",
                "-i",
                "-H",
                "Content-Type: text/plain",
                "--data-binary",
                "newword\n7000\nновоеслово\n",
            ])
            .output()
            .expect("Failed to execute command");
        let page_update = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        /*TODO uncomment after persistence layer added
        //get all words to check later if a new word is not rewritten by them
        let curl_output = Command::new("curl")
            .arg("http://localhost/words")
            .output()
            .expect("Failed to execute command");
        let page_all_words = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");
        */

        //check updated word
        let curl_output = Command::new("curl")
            .arg("http://localhost/words/newword")
            .output()
            .expect("Failed to execute command");
        let page_check_updated = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        //delete word
        let curl_output = Command::new("curl")
            .args(&["-X", "DELETE", "http://localhost/words/newword", "-i"])
            .output()
            .expect("Failed to execute command");
        let page_delete = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        //check if word exists
        let curl_output = Command::new("curl")
            .arg("http://localhost/words/newword")
            .output()
            .expect("Failed to execute command");
        let page_check_deleted = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        child.kill().expect("Failed to stop cargo");

        assert_eq!("Word \"newword\" is not found \u{1F622}", page_not_exist);
        assert_eq!("Word \"newword\" is added \u{1F60E}", page_add_new);
        assert_eq!(
            "Word \"newword\" is already exist \u{1F60E}",
            page_add_existing
        );
        assert!(page_update.starts_with("HTTP/1.1 204 No Content"));

        /*TODO uncomment after persistence layer added
        let words = vec!["testworda", "testwordb", "testwordc"];
        for word in words {
            assert!(page_all_words.contains(word));
        }
        assert!(page_all_words.contains("newword"));
        */

        assert!(page_check_updated.starts_with("newword<br \\>\n7000"));
        assert!(page_delete.starts_with("HTTP/1.1 204 No Content"));
        assert_eq!(
            "Word \"newword\" is not found \u{1F622}",
            page_check_deleted
        );
    } else {
        assert!(false);
    }
}
