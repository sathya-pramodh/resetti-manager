use argparse::{ArgumentParser, StoreTrue};
use std::{env::vars, path::Path, process::Command};
mod handlers;
use handlers::*;

fn main() {
    // Handling command line arguments by using the 'argparse' crate(https://crates.io/crates/argparse).
    let mut update_opt = false;
    let mut upgrade_opt = false;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("A binary manager for resetti.");
        parser.refer(&mut update_opt).add_option(
            &["-U", "--update"],
            StoreTrue,
            "Updates the indexes without installing them.",
        );
        parser.refer(&mut upgrade_opt).add_option(
            &["-u", "--upgrade"],
            StoreTrue,
            "Updates the indexes and installs them.",
        );
        parser.parse_args_or_exit();
    }

    // Get username of the logged in user.
    let username = match vars().find(|(var, _)| var == &"USER".to_string()) {
        Some((_, username)) => username,
        None => panic!("No logged in user found!"),
    };

    // Checking if ~/.cache/resetti-manager exists.
    let cache_dir = format!("/home/{}/.cache/resetti-manager", username);
    let cache_exists = Path::new(cache_dir.as_str()).is_dir();
    if !cache_exists {
        println!("Detected ~/.cache/resetti-manager does not exist. Creating...");
        match Command::new("mkdir")
            .arg("-p")
            .arg(cache_dir.as_str())
            .output()
        {
            Ok(_) => (),
            Err(err) => panic!("{}", err),
        };
        println!("Created ~/.cache/resetti-manager successfully!");
    }

    // Matching cases based on arguments passed.
    if update_opt {
        update(username)
    } else if upgrade_opt {
        upgrade(username)
    }
}
