#![warn(rust_2018_idioms)]

use bytes::{BufMut, BytesMut};
use futures::{stream::StreamExt};
use std::{env, io, str};
use tokio_util::codec::{Decoder, Encoder};
use futures::{Sink, SinkExt}; 
use tokio;
use tokio::sync::{mpsc, mpsc::Receiver};

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyACM0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let newline = src.as_ref().iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = src.split_to(n + 1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => {
                    Ok(Some(s.to_string()))
                },
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid String")),
            };
        }
        Ok(None)
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        println!("In writer {:?}", &item);
        dst.reserve(item.len() + 1);
        dst.put(item.as_bytes());
        dst.put_u8(b'\n');
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut args = env::args();
    let tty_path = args.nth(1).unwrap_or_else(|| DEFAULT_TTY.into());

    let settings = tokio_serial::SerialPortSettings::default();
    let mut port = tokio_serial::Serial::from_path(tty_path, &settings).unwrap();

    #[cfg(unix)]
    port.set_exclusive(false)
        .expect("Unable to set serial port exclusive to false");

    let mut io = LineCodec.framed(port);
    let (mut sink, mut stream) = io.split();

    
    while let Some(_) = stream.next().await {
        let _ = sink.send('C'.to_string()).await;
        break
    }
    let (mut tx, mut rx) = mpsc::channel(100);
    let j = tokio::spawn(async move {
        println!("ASYNC");
        test(&mut sink, &mut rx).await
    });

    println!("DONE");
    tx.send(true).await.expect("Error sending");
    tokio::join!(j).0.expect("Error");
}

async fn test(s: &mut (impl Sink<String> + Unpin), rx: &mut Receiver<bool>) {
    loop {
        let _ = s.send('A'.to_string()).await;
        std::thread::sleep_ms(1000);
        let _ = s.send('B'.to_string()).await;
        std::thread::sleep_ms(1000);
        if let Ok(v) = rx.try_recv() {
            if v {
                break;
            }
        }
    }
}