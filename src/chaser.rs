//! このファイルでは、CHaserクライアントの構造体と関数を定義しています。

use crate::action_result::ActionResult;
use crate::action_type::ActionType;
use crate::direction::Direction;
use std::io::{Error, Read, Write};
use std::net::TcpStream;

/// CHaserクライアントの構造体です。<br />
/// サーバー側との通信と、レスポンスの加工を定義しています。
pub struct CHaser {
    pub port: i32,
    pub address: String,
    pub stream: TcpStream,
    pub name: String,
}

impl CHaser {
    /// この構造体のコンストラクタです。
    fn new(address: String, port: i32, stream: TcpStream, name: String) -> CHaser {
        let a = CHaser {
            port,
            address,
            stream,
            name,
        };

        return a;
    }

    /// サーバーとの接続を確立し、インスタンスを返します。
    ///
    /// * `address` - サーバーのアドレス
    /// * `port` - サーバーのポート
    /// * `name` - 使用する名前
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

    /// GetReady のアクションを実行します。
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

    /// アクションを実行します。
    /// * `action` - アクションの種類
    /// * `dir` - アクションの方向
    pub fn do_action(&mut self, action: ActionType, dir: Direction) -> Result<ActionResult, Error> {
        let data: [u8; 4] = [action as u8, dir as u8, 13, 10];
        return self.send_byte(&data, true);
    }

    /// サーバーにデータを送信する関数です。
    /// * `send_data` - 送信する配列。
    /// * `end_code` - 通信の1ブロックの最後に`#\r\n`を送信するか。
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

    /// サーバーにデータを送信する関数です。
    /// * `str` - 送信する文字列。
    /// * `end_code` - 通信の1ブロックの最後に`#\r\n`を送信するか。
    fn send(&mut self, str: &str, end_code: bool) -> Result<ActionResult, Error> {
        return self.send_byte(str.as_bytes(), end_code);
    }

    /// サーバーとの通信を終了する関数です。
    fn close(&mut self) {
        match self.stream.shutdown(std::net::Shutdown::Both) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
