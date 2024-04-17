// chrono is a time library for Rust
use chrono::Datelike;

use rocket::serde::json::{Json};
// import our Date object from the routes/date module
use crate::routes::date::Date;
// Used for fheid custome syscalls
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::path::Path;
use tfhe::{FheUint32, set_server_key, CompressedServerKey, CompactFheUint32};
use std::str;
use tfhe::prelude::*;
use std::fs;

pub fn get_current_date() -> Date {
    #[derive(Deserialize, Debug)]
        struct Fheid {
            server_key: Vec<u8>,
            birth_date: Vec<u8>,
            today_date: u32
        }

        let file = File::open("/home/ubuntu/myapp/src/encryptData.json").unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `Fheid`.
        let u: Fheid = serde_json::from_reader(reader).unwrap();

        println!("File Read successfully");

        // Read the JSON encrypted data from the file
        let server_key_bytes = u.server_key;
        let birthday_bytes = u.birth_date;

        //Deserialize the encrypted data
        let compressed_sks: CompressedServerKey = bincode::deserialize(&server_key_bytes).unwrap();
        let birthday: CompactFheUint32 = bincode::deserialize(&birthday_bytes).unwrap();
        let today: u32 = u.today_date;

        println!("Deserialized Data successfully");

        //Decompress and set server key for doing encrypted execution
        let sks = compressed_sks.decompress();
        set_server_key(sks);

        println!("Set Server Key successfully");

        //Convert CompactFheUint32 to FheUint32 for doing encrypted calculations
        let birthday_fhe_uint32: FheUint32 = birthday.expand();
        let diff = today - birthday_fhe_uint32.clone();

        //Check if the person is an adult or not
        let encrypted_diff = &diff.gt(180000u32);

        println!("Computation done successfully");

        //Serialize the result to return back to the client
        let encrypted_res_bytes: Vec<u8> = bincode::serialize(&encrypted_diff).unwrap();

        // Store encrypted result into the file 
        let s =format!("{:?}", &encrypted_res_bytes.as_slice());
        let string = String::from("./src/encrypted_res.txt");
        let path = Path::new(&string);
        fs::write(path, s).unwrap();

        println!("Write data successfully");

        let current_utc = chrono::Utc::now();
        let year = current_utc.year();
        let month = current_utc.month();
        let day = current_utc.day();
        let current_date = Date {
            day,
            month,
            year
        };
        println!("Finish Request successfully");
        current_date
}

pub fn date_plus_month(mut date: Json<Date>) -> Date {
    // create mutable variable new_month and assign the given month + 1 to it
    let mut new_month = date.month + 1;
    // If new_month is over twelve (past December), set it to 1 (January)
    if new_month > 12 {
        new_month = 1;
        date.year = date.year + 1;
    }
    // create a new date object and return it
    let new_date = Date {
        day: date.day,
        month: new_month,
        year: date.year,
    };
    new_date
}
