use serialport;
use std::io;

fn main() {
    let mut t =
        serialport::open(&serialport::available_ports().expect("No serial port")[0].port_name)
            .expect("Faileed to open serial port");
    let mut buffer: [u8; 1] = [0; 1];
    loop {
        match t.read(&mut buffer) {
            Ok(bytes) => {
                if bytes == 1 {
                    t.write(&['C' as u8])
        .expect("Error sending data");
                    break;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }

    t.write(&['A' as u8])
        .expect("Error sending data");
    std::thread::sleep(std::time::Duration::from_secs(2));
}
