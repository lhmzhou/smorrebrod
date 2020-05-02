use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;

fn main() {
    loop {
        println!(
"< Hello! >
* Ready to develop in React?! This script will help you install the following:
* Homebrew
* zsh
* Oh-My-Zsh
* Zsh-nvm
* NVM
* HTTP proxies
* Node proxies
*
Before you proceed, make sure you have the following ready:
* HTTP and HTTPS proxies established
* Git
* Github access
* an ssh key linked to your github account
* Nodejs installed
*
Is the bash profile, Git, github access, and an ssh key set up? [Y/N]"
        );
        let mut confirmation = String::new();

        io::stdin()
            .read_line(&mut confirmation)
            .expect("Failed to read line");

        match &confirmation[..] {
            string if string.to_lowercase().contains('N') => return,
            string if string.to_lowercase().contains('Y') => {
                run_smorrebrod();
                break;
            }
            _ => continue,
        }
    }
}

// run child process with standard out piped to console
fn run_command_with_stdout(command_type: &str, arguments: &[&str], err_message: &str) {
    Command::new(command_type)
        .stdout(Stdio::inherit())
        .args(arguments)
        .output()
        .expect(err_message);
}

// run command without stdout or stdin
fn run_command(command_type: &str, arguments: &[&str], err_message: &str) {
    Command::new(command_type)
        .args(arguments)
        .output()
        .expect(err_message);
}

// run command and return the result of the command as a string
fn return_command_string(command_type: &str, arguments: &[&str], err_message: &str) -> String {
    let bytes = Command::new(command_type)
        .args(arguments)
        .output()
        .expect(err_message);

    return bytes.stdout.into_iter().map(|n| n as char).collect::<String>();
}

fn run_smorrebrod() {

    // lines is just for terminal seperator, makes it easier to see what's happening
    let lines = "============================================================================";

    // read the local enviroment variables into a string for later
    let env_vars = return_command_string("printenv", &[], "...printenv failed");

    /*
    *   @TODO parse it out with regex instead of being lazy and checking for individual strings
    */
    // if the contents of the environment variables do not have http/s proxies and the "proxy url" then panic
        if env_vars.contains("HTTP_PROXY") && env_vars.contains("HTTPS_PROXY") && env_vars.contains("localhost:8080") {
            println!("{}",lines);
            println!("Nice --- proxies are ready to go!", );
            println!("{}",lines);
        } else {
            panic!("Oops! We do not have the proxies", );
        }

        /*
         * @TODO make a network request after the proxies are set and make sure that everything is working. or else panic the program
         */
        match Command::new("brew").output() {
            Ok(_) => println!("brew already installed"),
            Err(_) => {

                // get ruby file for homebrew && turn to a string
                let ruby_file = return_command_string(
                    "curl",
                    &[
                        "-fsSL",
                        "https://raw.githubusercontent.com/Homebrew/install/master/install",
                    ],
                    "curl for hb",
                );
                
                // run command
                run_command_with_stdout(
                    "ruby",
                    &["-e", &ruby_file, "<", "/dev/null"],
                    "homebrew stallation failed",
                );
            }
        }
        println!("{}", lines);
        println!("INSTALLING NVM");
        match Command::new("nvm").output() {
            Ok(_) => println!("nvm installed!"),
            Err(_) => {
                match fs::create_dir(".nvm") {
                    Ok(_) => {
                        println!("Success! nvm directory created");
                        let nvm_script = return_command_string("curl", &["https://raw.githubusercontent.com/creationix/nvm/v0.34.0/install.sh"], "...curl for nvm failed");
                        fs::File::create("install_nvm.sh")
                            .expect("Creation of install_nvm.sh failed")
                            .write_all(nvm_script.as_bytes())
                            .expect("...writing to install_nvm.sh failed");
                        run_command("chmod", &["+x", "install_nvm.sh"], "chmod failed");
                        run_command("./install_nvm.sh", &[], "...nvm install failed");
                        fs::remove_file("install_nvm.sh").expect("...deletion of nvm_install.sh failed");
                    }
                    Err(_) => println!("~/.nvm file already exists! Skipping..."),
                };
            }
        }
        println!("{}", lines);
        println!("INSTALLING HOMEBREW PACKAGES");
    let brew_thread = thread::spawn(move || {

        // run command for homebew install packages
        run_command_with_stdout(
            "brew",
            &[
                "install",
                "zsh",
                "zsh-completions",
                "coreutils",
                "corkscrew",
                "gdbm",
                "htop-osx",
                "libtool",
                "pcre",
                "openssl",
                "libyaml",
                "nginx",
                "readline",
                "ruby",
                "wget",
                "mongodb",
                "docker",
                "docker-compose",
                "p7zip",
                "couchdb",
                "erlang",
                "geoip",
                "nettle",
                "redis",
            ],
            "Oops, brew install failed...",
        );
        run_command_with_stdout(
            "brew",
            &["cask", "install", "phantomjs"],
            "...brew cask install failed",
        );
    });

    let zsh_thread = thread::spawn(move || {
        println!("{}", lines);
        println!("INSTALLING OH-MY-ZSH");

        // install ohmyzsh
        match &Command::new("cd").args(&[".oh-my-zsh"]).output() {
            Ok(msg)
                if msg
                    .clone()
                    .stderr
                    .into_iter()
                    .map(|a| a as char)
                    .collect::<String>()
                    == "" =>
            {
                println!("Hey! Looks like oh-my-zsh is already installed: {:?}", msg)
            }
            Ok(_) => {
                let oh_m_z = return_command_string("curl", &["-fsSL", "https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh"], "curling oh-my-zsh failed");
                run_command_with_stdout("sh", &["-c", &oh_m_z], "...running zsh file failed")
            }
            Err(_) => println!("Error alert!"),
            _ => println!("hit default"),
        }

        println!("{}", lines);
    });

    brew_thread.join();
    nvm_thread.join();
    zsh_thread.join();
}
