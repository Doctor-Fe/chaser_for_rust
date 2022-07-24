use crate::action_result::ActionResult;
use crate::action_type::ActionType;
use crate::direction::Direction;
use std::io::{Error, Read, Write};
use std::net::TcpStream;

pub struct CHaser {
    pub port: i32,
    pub address: String,
    pub stream: TcpStream,
    pub name: String,
}

impl CHaser {
    fn new(address: String, port: i32, stream: TcpStream, name: String) -> CHaser {
        let a = CHaser {
            port,
            address,
            stream,
            name,
        };

        return a;
    }

    pub fn connect(address: String, port: i32, name: String) -> Result<CHaser, Error> {
        match TcpStream::connect(format!("{0}:{1}", &address, &port)) {
            Err(err) => {
                return Err(err);
            }
            Ok(mut stream) => match stream.write_all(format!("{}\r\n", name).as_bytes()) {
                Ok(_) => {
                    return Ok(CHaser::new(address, port, stream, name));
                }
                Err(err) => {
                    return Err(err);
                }
            },
        }
    }

    pub fn get_ready(&mut self) -> Result<ActionResult, Error> {
        let mut data: [u8; 3] = [0; 3];
        match self.stream.read(&mut data) {
            Ok(_) => {
                return self.send("gr\r\n", false);
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    pub fn do_action(&mut self, action: ActionType, dir: Direction) -> Result<ActionResult, Error> {
        let data: [u8; 4] = [action as u8, dir as u8, 13, 10];
        return self.send_byte(&data, true);
    }

    fn send_byte(&mut self, send_data: &[u8], end_code: bool) -> Result<ActionResult, Error> {
        let mut data: [u8; 12] = [0; 12];
        match self.stream.write_all(&send_data) {
            Ok(_) => match self.stream.read(&mut data) {
                Ok(_) => {
                    if end_code {
                        match self.stream.write_all("#\r\n".as_bytes()) {
                            Ok(_) => {
                                return Ok(ActionResult::new(data));
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        }
                    } else {
                        return Ok(ActionResult::new(data));
                    }
                }
                Err(error) => {
                    return Err(error);
                }
            },
            Err(error) => {
                self.close();
                return Err(error);
            }
        }
    }

    fn send(&mut self, str: &str, end_code: bool) -> Result<ActionResult, Error> {
        return self.send_byte(str.as_bytes(), end_code);
    }

    fn close(&mut self) {
        match self.stream.shutdown(std::net::Shutdown::Both) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
