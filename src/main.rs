/* Suspend server, first checking if there are any specific prgrams running
sleep 15 mins if program is runnng. Max tries = 10 before giving up.
This program can be called by a systemd timer to be run at a specific time e.g. midnight.
I am currenttly using cronie to run this!!!!!!!*/

mod config;
mod logger;
use logger::logger;
use std::process::Command;
//use std::str::FromStr;
use std::{thread, time};
use std::path::PathBuf;

fn main() {
    //pgrep is used to check if any programs in the lsit are currentl running#
    //let progs_check = ["brave", "borg", "borgmatic"]; //put this in a config file
    let mut suspend_ok: bool = false;
    const LOG_FILE: &str = "suspend-server.log";
    let mut tries: u64 = 0;

    //config file
    let mut conf = config::ConfigFile::default(); //get the config
    match conf.get_config("/etc/"){
        Ok(..) => (),
        Err(err) => {panic!("{}", format!("Error in config file {}",err));}
    }

    let mut log_file_pb = PathBuf::from(conf.log_path);
    log_file_pb.push(LOG_FILE);
    let log_file = log_file_pb.as_os_str();

    if conf.programs_list.len() == 0{
        logger(log_file,"Error in config file - no programs to check");
        panic!("Error in config file - no programs to check");
    }

    println!("programs = {}", conf.programs_list);
    println!("Log file = {:?}", log_file);
    println!("Max tries = {}",conf.max_tries);
    println!("delay = {}",conf.delay);

    let progs_iter = conf.programs_list.split(","); //get iterator for programs to check
    let progs = progs_iter.collect::<Vec<_>>();
    //println!("progs = {:?}", progs);
    while (tries < conf.max_tries) && !suspend_ok{
        tries += 1;
        suspend_ok=true;
        for prog in progs.iter(){
            logger(log_file, format!("Checking for {}",prog).as_str());
            println!("Checking for {}", prog);
            let mut sh_command = Command::new("pgrep");
            sh_command.arg("-f");
            sh_command.arg(prog);
            let sh_output = sh_command.output().expect("failed to process pgrep"); //handle this better - log it! (Result)
            let progs = String::from_utf8(sh_output.stdout).unwrap(); // to do - handle this error
            //println!("Programs detected: {}",progs);
            if progs.len() > 0{
                logger(log_file, format!("{} is running, can't suspend", prog).as_str());
                println!("{} is running, can't suspend", prog);
                suspend_ok = false;
                //sleep for 15 mins
                let duration = time::Duration::from_secs(conf.delay); //wait 15 mins
                thread::sleep(duration);
                break;
            }
        }
    }

    if suspend_ok{
        logger(log_file, "systemctl suspend");
        println!("systemctl suspend");
        let mut sh_command = Command::new("systemctl");
        sh_command.arg("suspend");
        let sh_output = sh_command.output().expect("failed execute systemctl suspend"); //handle this better - log it! (Result)
        let output = String::from_utf8(sh_output.stdout).unwrap(); // to do - handle this error
        logger(log_file, output.as_str());
    }
    else{
        logger(log_file, "Conditions not met to suspend system");
        println!("Conditions not met to suspend system");
    }

}
