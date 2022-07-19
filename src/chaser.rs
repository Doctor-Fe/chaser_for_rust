use std::net::TcpStream;
use std::io::{Error, Write, Read};

pub struct CHaser {
    pub port: i32,
    pub address: &'static str,
    pub stream: TcpStream,
    pub name: &'static str
}

impl CHaser {

    pub fn new(address: &'static str, port:i32, stream:TcpStream, name: &'static str) -> CHaser
    {
        let a = CHaser {
            port,
            address,
            stream,
            name
        };

        return a;
    }

    pub fn connect(address: &'static str, port: i32, name: &'static str) -> Result<CHaser, Error>
    {
        let result: Result<TcpStream, Error> = TcpStream::connect(format!("{}:{}", &address, &port));
        match result {
          Err(err) => {
            return Err(err);
          }
          Ok(stream) => {
            return Ok(CHaser::new(address, port, stream, name));
          }
        }
    }

    pub fn get_ready(&mut self) -> Result<[i32; 9], Error>
    {
      match self.stream.write_all(b"name") {
        Ok(_) => {
          let mut data: Vec<u8> = Vec::new();
          match self.stream.read_to_end(&mut data) {
            Ok(r) => {
              data.get(0..10);
              // TODO データ受信に成功した時の処理。
            },
            Err(error) => {
              return Err(error);
            }
          }
        },
        Err(error) => {
          return Err(error);
        }
      }
        return Ok([1,2,3,4,5,6,7,8,9]);
    }
}