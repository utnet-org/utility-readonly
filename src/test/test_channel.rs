use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[test]
pub fn test_channel()
{
    let (tx,rx) = mpsc::channel();
    thread::spawn(move ||{
        let val = String::from("hello");
        tx.send(val).unwrap();
        println!("thread start");
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

#[test]
pub fn test_channel_many()
{
    let (tx,rx) = mpsc::channel();
    thread::spawn(move ||{
        let v = vec![
            String::from("hello1"),
            String::from("hello2"),
            String::from("hello3"),
            String::from("hello4"),
        ];

        for val in v {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(3));
        }
    });
    for received in rx{
        println!("Got: {:?}", received);
    }
}