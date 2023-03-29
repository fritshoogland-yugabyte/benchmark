use std::fs;
//use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::process;

fn main() {
    let file_name = "/tmp/testfile";
    //let data = "x".to_string().repeat(102400);
    // 67108864 = 64MB
    let data = vec![0_u8; 67108864];
    //let choice = "create-write-fsync";
    //let choice = "create-write-overwrite-fsync";
    let choice = "create-write-fdatasync";
    //let choice = "create-write-overwrite-fdatasync";

    println!("Ready pid={}", process::id());
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed");

    
    match choice
    {
        "create-write-fsync" => {
            fs::remove_file(&file_name)
                  .unwrap_or_else(|_| println!("File did not exist"));
            let mut f = fs::OpenOptions::new()
                  .create(true)
                  .write(true)
                  .open(&file_name)
                  .expect("Unable to open file for write");
            f.write_all(&data)
                  .expect("Unable to write data");
            f.sync_all()
                  .expect("Unable to fsync");
        },
        "create-write-overwrite-fsync" => {
            fs::remove_file(&file_name)
                  .unwrap_or_else(|_| println!("File did not exist"));
            let mut f = fs::OpenOptions::new()
                  .create(true)
                  .write(true)
                  .open(&file_name)
                  .expect("Unable to open file for write");
            f.write_all(&data)
                  .expect("Unable to write data");
            f.sync_all()
                  .expect("Unable to fsync");
            let mut f = fs::OpenOptions::new()
                  .append(false)
                  .write(true)
                  .open(&file_name)
                  .expect("Unable to open file for write");
            f.write_all(&data)
                  .expect("Unable to write data");
            f.sync_all()
                  .expect("Unable to fsync");
        },
        "create-write-fdatasync" => {
            fs::remove_file(&file_name)
                  .unwrap_or_else(|_| println!("File did not exist"));
            let mut f = fs::OpenOptions::new()
                  .create(true)
                  .write(true)
                  .open(&file_name)
                  .expect("Unable to open file for write");
            f.write_all(&data)
                  .expect("Unable to write data");
            f.sync_data()
                  .expect("Unable to datafsync");
        },
        "create-write-overwrite-fdatasync" => {
            fs::remove_file(&file_name)
                  .unwrap_or_else(|_| println!("File did not exist"));
            let mut f = fs::OpenOptions::new()
                  .create(true)
                  .write(true)
                  .open(&file_name)
                  .expect("Unable to open file for write");
            f.write_all(&data)
                  .expect("Unable to write data");
            f.sync_data()
                  .expect("Unable to datafsync");
            let mut f = fs::OpenOptions::new()
                  .append(false)
                  .write(true)
                  .open(&file_name)
                  .expect("Unable to open file for write");
            f.write_all(&data)
                  .expect("Unable to write data");
            f.sync_data()
                  .expect("Unable to datafsync");
        },
        _ => println!("invalid choice"),
    }
}
