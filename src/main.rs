/* Suspend server, first checking if there are any specific prgrams running
sleep 15 mins if program is runnng. Max tries = 10 before giving up.
This program can be called by a systemd timer to be run at a specific time e.g. midnight.*/
mod logger;
mod config;
use logger::logger;
use std::process::Command;
use std::{thread, time};

fn main() {
    //pgrep is used to check if any programs in the lsit are currentl running#
    //let progs_check = ["brave", "borg", "borgmatic"]; //put this in a config file
    let mut suspend_ok: bool = false;
    const MAX_TRIES: i32 = 10;
    let mut tries: i32 = 0;

    //config file
    let mut conf = config::ConfigFile::default(); //get the config
    conf.get_config("/etc/");

    if conf.programs_list.len() == 0{
        println!("Error in config file - no programs to check");
        return;
    }

    let progs_iter = conf.programs_list.split(","); //get iterator for programs to check
    let progs = progs_iter.collect::<Vec<_>>();
    println!("progs = {:?}", progs);
    while (tries < MAX_TRIES) && !suspend_ok{
        tries += 1;
        suspend_ok=true;
        for prog in progs.iter(){
            println!("Checking for {}", prog);
            let mut sh_command = Command::new("pgrep");
            sh_command.arg(prog);
            let sh_output = sh_command.output().expect("failed to process pgrep"); //handle this better - log it! (Result)
            let progs = String::from_utf8(sh_output.stdout).unwrap(); // to do - handle this error
            //println!("Programs detected: {}",progs);
            if progs.len() > 0{
                println!("{} is running, can't suspend", prog);
                suspend_ok = false;
                //sleep for 15 mins
                let duration = time::Duration::from_secs(10);
                thread::sleep(duration);
                break;
            }
        }
    }

    if suspend_ok{
        println!("systemctl suspend");
    }
    else{
        println!("Conditions not met to suspend system");
    }

}
