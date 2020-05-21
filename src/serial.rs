#![allow(dead_code)]

use serialport;
use std::io;

pub struct SerialPort {
    port: Box<dyn serialport::SerialPort>,
}

#[allow(dead_code)]
impl SerialPort {
    pub fn new() -> Self {
        Self {
            port: serialport::open(
                &serialport::available_ports().expect("No serial port")[0].port_name,
            )
            .expect("Faileed to open serial port"),
        }
    }

    pub fn read_string(&mut self) -> String {
        let mut r = String::new();
        while self
            .port
            .bytes_to_read()
            .expect("Error getting available bytes")
            > 0
        {
            let mut buf = [0; 1];
            match self.port.read(&mut buf) {
                Ok(_) => r.push(buf[0] as char),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            };
        }
        return r;
    }

    pub fn readline(&mut self) -> String {
        let mut r = String::new();
        loop {
            let mut buf = [0; 1];
            match self.port.read(&mut buf) {
                Ok(_) => {
                    if buf[0] as char == '\n' {
                        break
                    }
                    r.push(buf[0] as char);
                }, 
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            };
        }
        return r.trim_end().to_string();
    }

    pub fn write_string<T: ToString>(&mut self, string: T) -> std::result::Result<(), std::io::Error> {
        let mut s = string.to_string().clone();
        while !s.is_empty() {
            self.port.write(&[s.pop().unwrap() as u8])?;
        }
        Ok(())
    }

    pub fn write_line<T: ToString>(&mut self, string: T) -> std::result::Result<(), std::io::Error>{
        self.write_string(string.to_string() + "\n")
    }

    fn clone(&self) -> Self {
        Self {
            port: self.port.try_clone().expect("Error duplicating port"),
        }
    }

    pub fn split(&self) -> (SerialReader, SerialWriter) {
        (
            SerialReader {
                ser: Box::new(self.clone()),
            },
            SerialWriter {
                ser: Box::new(self.clone()),
            },
        )
    }
}

impl io::Read for SerialPort {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        self.port.read(buf)
    }
}

impl io::Write for SerialPort {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self.port.write(buf)
    }
    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        self.port.flush()
    }
}

pub struct SerialReader {
    ser: Box<SerialPort>,
}

impl SerialReader {
    pub fn read_string(&mut self) -> String {
        self.ser.read_string()
    }

    pub fn readline(&mut self) -> String {
        self.ser.readline()
    }
}

impl Clone for SerialReader {
    fn clone(&self) -> Self {
        Self {
            ser: Box::new(self.ser.clone()),
        }
    }
}

impl io::Read for SerialReader {
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        self.ser.read(buf)
    }
}

pub struct SerialWriter {
    ser: Box<SerialPort>,
}

impl SerialWriter {
    pub fn write_string<T: ToString>(&mut self, string: T) -> std::result::Result<(), std::io::Error> {
        self.ser.write_string(string)
    }

    pub fn write_line<T: ToString>(&mut self, string: T) -> std::result::Result<(), std::io::Error>{
        self.ser.write_line(string)
    }
}

impl io::Write for SerialWriter {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self.ser.write(buf)
    }
    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        self.ser.flush()
    }
}
