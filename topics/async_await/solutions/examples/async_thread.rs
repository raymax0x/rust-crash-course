#![allow(unused)]

use std::time::Duration;
use tokio::time::sleep;

// threads vs async
//  when to use threads vs async / await ?

#[tokio::main]
async fn main() {
    // Spawning too many threads can crash this program (OS thread and memory limits)

    let mut handles = vec![];

    // bad way to do it
    // for i in 0..1000000 {
    //     handles.push(std::thread::spawn(move || {
    //         std::thread::sleep(Duration::from_millis(100));
    //         println!("{i}: 🍔 is ready");
    //     }));
    // }
    //  for h in handles {
    //     h.join().unwrap();
    // }

   for i in 0..1000000 {
        let fut  = async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("{i}: 🍔 is ready");
        };

        let handler = tokio::task::spawn(fut);
        handles.push(handler);
    }

    for h in handles {
        h.await.unwrap();
    }


   
}
