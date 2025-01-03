use std::fs;
//use std::fs::read_dir;
use std::path::PathBuf;
use std::string::String;


#[derive(Default)]
pub struct ConfigFile{
    pub programs_list: String,
    pub log_path: String,
}

impl ConfigFile{
    pub fn get_config(&mut self, conf_path: &str) -> Result<(), String>{
        let mut file_path = PathBuf::from(conf_path);
        const CONFIG_FILE: &str = "suspend-server.conf";
        file_path.push(CONFIG_FILE);
        //println!("conf file = {:?}",file_path);
        let contents = ConfigFile::read_config_file(file_path)?;
        let lines = contents.split("\n");
        for line in lines{
            if line.starts_with("programs ="){
                self.programs_list = ConfigFile::get_value_from_line(line)?.to_string();
            }
            else if line.starts_with("log path ="){
                self.log_path = ConfigFile::get_value_from_line(line)?.to_string().trim().parse().unwrap();
            }
            else {
                return Err(format!("Found unrecognised config line: {}",line));
            }
        }

        Ok(())
    }

    fn get_value_from_line(line: &str) -> Result<&str,&str>{
        let mut conf_line_parts = line.split("=");
        if conf_line_parts.clone().count() != 2{  //Note clone() to make acopy as count would consume conf_line_parts iterator
            return Err("Bad config file: invalid format");
        }

        //let value: &str = conf_line_parts.nth(1).unwrap().trim(); // note .unwrap() returns value of 'Some' from Option type (Some and None).
        let value: &str;
        match conf_line_parts.nth(1){
            Some(val) => {value = val.trim();}
            None => {return Err("Couldn't get value from config file past '='");}
        }
        Ok(value)
    }

    fn read_config_file(config_file: PathBuf) -> Result<String, String>{
        //println!("Reading config file...");
        // As reading file line by line is not working, lets read it into a string
        //let contents = fs::read_to_string(config_file).expect("Could not read config file (empty)");
        let contents: String;
        match fs::read_to_string(config_file){
            Ok(s) => contents = s,
            Err(_e) => {return Err(String::from("Could not read config file"));}
        }
        Ok(contents)
    }
}

