
use std::net::{TcpStream, TcpListener};
use std::io::{Write, BufReader, BufRead, Read};
use protobuf::{Message, ProtobufResult};
use proto::fileBodys::FileBody;


pub struct Rsocket {
    pub host_and_port: String,
    pub object: Vec<u8>,
}


impl Rsocket {
    pub fn new(&self) -> Rsocket {
        Rsocket {
            host_and_port: String::from(""),
            object: vec![],
        }
    }

    pub fn send_msg(&mut self, pojo: &Vec<u8>) -> FileBody {
        let mut send_msg: TcpStream = TcpStream::connect(&self.host_and_port).unwrap();
        send_msg.write(&mut self.object);
        self.read_msg(&mut send_msg)
    }

    fn read_msg(&self, send_msg: &mut TcpStream) -> FileBody {
        let mut read_bytes = [0; 102400];
        let byte_len: usize = send_msg.read(&mut read_bytes).expect("");
        let boyd: ProtobufResult<FileBody> = Message::parse_from_bytes(&read_bytes[0..byte_len]);
        boyd.expect("no no no....")
    }
}