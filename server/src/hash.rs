extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash,verify};


pub(crate) fn hash_password(){
    let hashed = hash("hunter2", DEFAULT_COST).unwrap();
    let valid = verify("hunter2", &hashed).unwrap();
    println!("{:?}", valid)
}