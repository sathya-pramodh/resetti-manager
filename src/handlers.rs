use std::{path::Path, process::Command};

// install: Installs resetti into `/usr/bin`
fn install(updated_bin: String) {
    match Command::new("sudo")
        .arg("install")
        .arg("-Dm0755")
        .arg(updated_bin.as_str())
        .arg("/usr/bin/resetti")
        .output()
    {
        Ok(_) => (),
        Err(err) => panic!("Could not copy binary to `/usr/bin`: {}", err),
    }
}

// update: Updates the indexes without installing them.
// TODO: Add bench script download when it is bundled with latest release.
pub fn update(username: String) {
    let url = "https://github.com/woofdoggo/resetti/releases/latest/download/resetti";
    let bin_path = format!("/home/{}/.cache/resetti-manager/", username);

    // Delete the binary if it already exists.
    let bin_exists = Path::new(bin_path.as_str()).is_file();
    if bin_exists {
        match Command::new("rm")
            .arg((bin_path.to_owned() + "resetti").as_str())
            .output()
        {
            Ok(_) => (),
            Err(err) => panic!("Unable to remove file: {}", err),
        }
    }

    // Download the binary from the latest release.
    match Command::new("sh")
        .arg("-c")
        .arg(format!("wget {}", url))
        .output()
    {
        Ok(_) => (),
        Err(err) => panic!("Unable to download from url: {}", err),
    }
    match Command::new("sh")
        .arg("-c")
        .arg(format!("mv resetti {}", bin_path.as_str()))
        .output()
    {
        Ok(_) => (),
        Err(err) => panic!("Unable to move binary: {}", err),
    }

    // Change executable permissions on the newly downloaded binary.
    match Command::new("chmod")
        .arg("+x")
        .arg((bin_path.to_owned() + "resetti").as_str())
        .output()
    {
        Ok(_) => (),
        Err(err) => panic!("Unable to change permissions on file: {}", err),
    };
}

// upgrade: Updates the indexes and installs them.
pub fn upgrade(username: String) {
    // Update indexes first.
    update(username.to_owned());

    // Check if binary exists in `/usr/bin`.
    let updated_bin = format!("/home/{}/.cache/resetti-manager/resetti", username);
    let bin_exists = Path::new("/usr/bin/resetti").is_file();
    if bin_exists {
        // Check if the version has changed.
        let latest_version = match Command::new("sh")
            .arg("-c")
            .arg(updated_bin.as_str())
            .arg("version")
            .output()
        {
            Ok(out) => match String::from_utf8(out.stdout) {
                Ok(st) => st,
                Err(err) => panic!("Unknown UTF-8 error: {}", err),
            },
            Err(err) => panic!("Could not get the version of the latest binary: {}", err),
        };
        let current_version = match Command::new("sh")
            .arg("-c")
            .arg("/usr/bin/resetti")
            .arg("version")
            .output()
        {
            Ok(out) => match String::from_utf8(out.stdout) {
                Ok(st) => st,
                Err(err) => panic!("Unknown UTF-8 error: {}", err),
            },
            Err(err) => panic!("Could not get the version of the current binary: {}", err),
        };
        if latest_version != current_version {
            // Install the binary to `/usr/bin`.
            install(updated_bin);
        }
    } else {
        // Install the binary to `/usr/bin`.
        install(updated_bin);
    }
}
