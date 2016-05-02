// #![allow(dead_code)]
// #![allow(unused_imports)]
extern crate labyrinth as library;
extern crate num_cpus;
extern crate threadpool;

use std::env;
use threadpool::ThreadPool;
#[allow(unused_imports)]
use std::sync::mpsc::{channel, sync_channel};

use library::{Int, Uint, Wall};

fn main() {
    let mut args = env::args();
    args.next();
    let cols     = if let Some(arg) = args.next() { arg.parse::<Int>() }   else { Ok(10) };
    let rows     = if let Some(arg) = args.next() { arg.parse::<Int>() }   else { Ok(10) };
    let deep     = if let Some(arg) = args.next() { arg.parse::<Uint>() }  else { Ok(1u32) };
    let poolsize = if let Some(arg) = args.next() { arg.parse::<usize>() } else { Ok(num_cpus::get()) };

    match (rows, cols, deep, poolsize) {
        (Ok(rows), Ok(cols), Ok(deep), Ok(poolsize)) => {
            if rows > 0 && cols > 0 {

                /* Linear implementation */
                // for _ in 0..deep {
                //     let mut wall = Wall::new(cols, rows);
                //     wall.carve();
                //     wall.print();
                //     println!("");
                // }

                /* Parallel implementation */
                let pool = ThreadPool::new(poolsize);
                let port = {
                    let (chan, port) = channel(); // sync_channel((deep as usize + poolsize - 1) / poolsize);
                    for _ in 0..deep {
                        let chan = chan.clone();
                        pool.execute(move || {
                            let mut wall = Wall::new(cols, rows);
                            wall.carve();
                            chan.send(wall).unwrap();
                        });
                    }
                    port
                };
                for _ in 0..deep {
                    let wall = port.recv().unwrap();
                    wall.print();
                }

            } else {
                println!("Hey, give me numbers grater than 0");
            }
        }
        _ => println!("Hey, give me a number.")
    }
}