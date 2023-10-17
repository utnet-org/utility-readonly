use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
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
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx{
        println!("Got: {:?}", received);
    }
}
