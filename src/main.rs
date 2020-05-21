#[allow(unused_imports)]
use krpc_mars;
mod krpc;
mod serial;
mod protocol;

#[allow(unused_imports)]
use std::io::{Read, Write, self};

fn main() {
    println!("Hello, world!");
    
    // let client = krpc_mars::RPCClient::connect("Example", "127.0.0.1:50000").expect("Error connecting");
    // println!("Connected");
    // let v = client.mk_call(&krpc::space_center::get_game_mode()).expect("ede");
    // println!("{:?}",v);
    // loop {}


    let mut t = serial::SerialPort::new();
    let mut buffer: [u8; 3] = [0; 3];
    loop {
        match t.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 1 {
                    t.write(&['C' as u8]).expect("Error sending data");
                    break;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
    let (mut reader, mut writer) = t.split();
    std::thread::sleep(std::time::Duration::from_secs(4));
    writer.write_all(&protocol::Value::Lcdwrite("Hello,".to_string(), "world!".to_string()).repr()).expect("Error writing to string");
    std::thread::sleep(std::time::Duration::from_secs(4));
    println!("ON");
    writer.write_all(&protocol::Value::Led_1(true).repr()).expect("Error writing to string");
    
    std::thread::sleep(std::time::Duration::from_secs(10));
    // std::thread::spawn(move || {
    //     let mut v = String::new();
    //     loop {
    //         let new_v = reader.readline();
    //         if v != new_v {
    //             println!("{}", v);
    //             v = new_v;
    //         }
    //     }
    // });
    // loop {
    //     writer.write_string("A").expect("Error sending data");

    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //     writer.write_string("B").expect("Error sending data");

    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    // }
}
