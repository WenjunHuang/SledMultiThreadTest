extern crate clap;
extern crate sled;

use clap::{App, Arg, SubCommand};
use sled::*;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::*;

fn main() {
    let matches = App::new("Sled Multi-threaded test")
        .arg(Arg::with_name("mode")
            .short("m")
            .takes_value(true)
            .required(true)
            .help("Test write only"))
        .arg(Arg::with_name("threads").short("t")
            .help("threads count")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("INPUT")
            .help("Sled directory")
            .required(true)
            .index(1))
        .get_matches();

    let thread_count = i32::from_str(matches.value_of("threads").unwrap()).unwrap();
    let mode = matches.value_of("mode").unwrap();
    let dir = matches.value_of("INPUT").unwrap();

    let db = Arc::new(Tree::start_default(dir).expect("sled must start ok"));
    if mode == "r" {
        do_read_test(db.clone(), thread_count);
    }
    else if mode=="w"  {
        do_write_test(db.clone(), thread_count);
    }else if mode=="wr" {
        do_write_test(db.clone(), thread_count);
        do_read_test(db.clone(), thread_count);
    }
}

fn do_write_test(db: Arc<Tree>, thread_count: i32) {
    println!("starting do write test...");
    let mut threads = vec![];
    for i in 0..thread_count {
        let d = db.clone();
        let t = spawn(move || {
            check_write(d, i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
    println!("do write test succeeded.");
}

fn check_write(db: Arc<Tree>, prefix: i32) {
    let mut key = vec!(b'0', b'0', b'0', b'0', b'0', b'e', b'n', b'd');// 8 bytes key
    let value = vec![b'6'; 4096];

    for i in 0..10000 {
        let mut v = value.clone();
        for j in 0..4 {
            key[j] = ((i >> ((4 - j - 1) << 3)) & 0xFF) as u8;
        }
        unsafe {
            let p = v.as_mut_ptr() as *mut i32;
            *p = i as i32;
        }

        let result = db.set(key.clone(), v);
        match result {
            Ok(_) => {}
            Err(cause) => {
                eprintln!("error in write: {:?}", cause);
                std::process::exit(-1);
            }
        }
    }
}

fn check_read(db: Arc<Tree>, prefix: i32) {
    let mut key = vec!(b'0', b'0', b'0', b'0', b'0', b'e', b'n', b'd');// 8 bytes key
    let suffix = vec![b'6'; 4092];

    for i in 0..10000 {
        for j in 0..4 {
            key[j] = ((i >> ((4 - j - 1) << 3)) & 0xFF) as u8;
        }
        let result = db.get(&key);
        match result {
            Ok(Some(ref pin)) => {
                let rv = pin.to_vec();
                // is length correct?
                if rv.len() != 4096 {
                    eprintln!("value len is not right");
                    std::process::exit(-1);
                }

                // is magic number correct?
                unsafe {
                    let p = rv.as_ptr() as *const i32;
                    if *p != i {
                        eprintln!("value magic for key:{:?} is {} ,but expected is {}",
                                  key, *p, i);
                        std::process::exit(-1);
                    }
                }

                // is data correct?
                let (_, rest) = rv.split_at(4);
                if rest.cmp(suffix.borrow()) != Ordering::Equal {
                    eprintln!("value for key:{:?} is not the same as written", key);
                    std::process::exit(-1);
                }
            }
            Ok(None) => {
                eprintln!("key not found: {:?}", key);
                std::process::exit(-1);
            }
            Err(_) => {
                std::process::exit(-1);
            }
        }
    }
}

fn do_read_test(db: Arc<Tree>, thread_count: i32) {
    println!("starting do read test.....");
    let mut threads = vec![];
    for i in 0..thread_count {
        let d = db.clone();
        let t = spawn(move || {
            check_read(d, i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
    println!("do read test succeeded.");
}
