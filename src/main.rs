use curl::easy::Easy;
use std::{fs, fs::File, io::Write};
use chrono::prelude::*;



#[path = "config_structs.rs"]
mod config_structs;

struct DlResult {
    response_code: i32,
    message: String,
    error_message: String
}

fn do_download(url: String, _outfile_name: String) -> DlResult {

    let mut ret = DlResult { response_code: 0, message: "not implemented".to_string(), error_message: "".to_string() };
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&url).unwrap();
    
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let mut outfile = File::create(_outfile_name).expect("  [!] unable to create output file");
    match outfile.write_all(&data) {
        Ok(()) => return ret,
        Err(err) => {
            ret.response_code = -1;
            ret.message = "error downloading file".to_string();
            ret.error_message = err.to_string();
            return ret;
        }
    }
    
}

fn main() {
    
    let dtnow = Utc::now();
    println!("[*] starting...");

    let config_str = match std::fs::read_to_string("_config.json") { 
        Ok(c) => c.to_string(),
        Err(e) => panic!("[!] cannot read config file: {}",e)
    };

    let config: config_structs::Root = serde_json::from_str(&config_str).expect("Error loading config data");
    let subdir = format!("{}/{}", "ti_downloads", dtnow.format("%Y%m%d%H%M%S"));
    
    match fs::create_dir_all(&subdir) {
        Ok(()) => println!("[*] created subdir {}", &subdir),
        Err(e) => panic!("[!] cannot create output dir: {}", e)
    }

    
    for feed in config.feeds { 
        let sd = &subdir;
        let url = feed.url;
        let outfile = sd.to_owned()+"/"+&feed.outfile_name;
        println!("  [*] Downloading {} from {}...", outfile, url);
        let dlresult = do_download(url.to_string(), outfile.to_string());
        if dlresult.response_code == 0 {
            println!("      -> Success!")
        }else{
            println!("    [!] Error occured: {}", dlresult.error_message);
        }
    }

    println!("[.] Done.")


}
