//@author Stanislav Polaniev <spolanyev@gmail.com>

use std::process::Command;
use std::thread;
use std::time::Duration;

fn validate_data(data: &str) -> Result<(), &'static str> {
    let lines: Vec<&str> = data.split('\n').collect();

    if lines.len() != 3 {
        return Err("Data must contain exactly three lines");
    }

    if lines[0].is_empty() {
        return Err("Word must be a non-empty string");
    }

    if lines[1].parse::<i32>().is_err() {
        return Err("Frequency must be a valid integer");
    }

    if !lines[2].is_char_boundary(lines[2].len()) {
        return Err("Translation must be valid UTF-8 encoded text");
    }

    Ok(())
}

#[test]
fn add_update_delete() {
    //set env=test
    let mut cargo = Command::new("cargo");
    if let Ok(mut child) = cargo.arg("run").env("RUST_ENV", "test").spawn() {
        thread::sleep(Duration::from_secs(1)); //time to start server

        //make sure test env is set
        let curl_output = Command::new("curl")
            .arg("http://localhost/health")
            .output()
            .expect("Failed to execute command");
        let page_test_env = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        if !page_test_env.contains("RUST_ENV=test") {
            child.kill().expect("Failed to stop cargo");
            assert!(false);
        }

        //sure not exist
        let curl_output = Command::new("curl")
            .arg("http://localhost/words/newword")
            .output()
            .expect("Failed to execute command");
        let page_not_exist = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        //add new word
        let data = "newword
            5000
            новое слово";

        match validate_data(data) {
            Ok(()) => {}
            Err(error) => eprintln!("[31mWrong data[0m {error:#?}"),
        }

        let curl_output = Command::new("curl")
            .args(&[
                "-X",
                "POST",
                "http://localhost/words",
                "-H",
                "Content-Type: text/plain",
                "--data-binary",
                data,
            ])
            .output()
            .expect("Failed to execute command");

        let page_add_new = {
            if !curl_output.status.success() {
                let error_output = String::from_utf8(curl_output.stderr)
                    .expect("Failed to convert stderr to String");
                eprintln!("Curl failed with error: {}", error_output);
                String::new()
            } else {
                let page_add_new = String::from_utf8(curl_output.stdout)
                    .expect("Failed to convert stdout to String");
                //println!("Curl output: {}", page_add_new);
                page_add_new
            }
        };

        //add existing word
        let data = "newword
            5000
            новое слово";

        match validate_data(data) {
            Ok(()) => {}
            Err(error) => eprintln!("[31mWrong data[0m {error:#?}"),
        }

        let curl_output = Command::new("curl")
            .args(&[
                "-X",
                "POST",
                "http://localhost/words",
                "-H",
                "Content-Type: text/plain",
                "--data-binary",
                data,
            ])
            .output()
            .expect("Failed to execute command");
        let page_add_existing = String::from_utf8(curl_output.stdout.as_slice().to_owned())
            .expect("Failed to convert to String");

        //update word
        let str = "newword
            7000
            новоеслово";

        match validate_data(str) {
            Ok(()) => {}
            Err(error) => eprintln!("[31mWrong data[0m {error:#?}"),
        }

        let curl_output = Command::new("curl")
            .args(&[
                "-X",
                "PUT",
                "http://localhost/words",
                "-i",
                "-H",
                "Content-Type: text/plain",
                "--data-binary",
                str,
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
