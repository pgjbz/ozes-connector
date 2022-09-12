use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::errors::OzesResult;

pub struct Publisher<T>
where
    T: Read + Write,
{
    stream: T,
}

pub type OzPublisher = Publisher<TcpStream>;

pub struct PublisherBuilder<'s> {
    queue_name: &'s str,
    port: u16,
    host: &'s str,
}

impl<'a> Default for PublisherBuilder<'a> {
    fn default() -> Self {
        Self {
            queue_name: "local_queue",
            port: 7656,
            host: "localhost",
        }
    }
}

impl<'a> PublisherBuilder<'a> {
    pub fn with_host(mut self, host: &'a str) -> Self {
        self.host = host;
        self
    }

    pub fn on_queue(mut self, queue_name: &'a str) -> Self {
        self.queue_name = queue_name;
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn build(self) -> OzesResult<OzPublisher> {
        Publisher::new(self)
    }
}

impl Publisher<TcpStream> {
    pub fn builder<'s>() -> PublisherBuilder<'s> {
        PublisherBuilder::default()
    }

    fn new(builder: PublisherBuilder) -> OzesResult<Self> {
        let address = format!("{}:{}", builder.host, builder.port);
        let mut stream = TcpStream::connect(address)?;
        stream.write_all(format!("PUBLISHER {};", builder.queue_name).as_bytes())?;
        crate::unwrap_return(&mut stream)?;
        Ok(Self { stream })
    }
}

impl<T: Read + Write> Publisher<T> {
    pub fn send_message(&mut self, message: &[u8]) -> OzesResult<()> {
        let mut vec = Vec::with_capacity(message.len() + 10);
        vec.extend_from_slice(b"message \"");
        vec.extend_from_slice(message);
        vec.push(b'"');
        self.stream.write_all(&vec)?;
        crate::unwrap_return(&mut self.stream)
    }

    pub fn send_binary(&mut self, message: &[u8]) -> OzesResult<()> {
        let mut vec = Vec::with_capacity(message.len() + 9);
        vec.extend_from_slice(b"message #");
        vec.extend_from_slice(message);
        self.stream.write_all(&vec)?;
        crate::unwrap_return(&mut self.stream)
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_send_binary() {
        let ok_message = b"ok message";
        let mut read_data = Vec::with_capacity(ok_message.len());
        read_data.extend_from_slice(ok_message);
        let mock_tcpstream = MockTcpStream {
            read_data,
            write_data: vec![],
        };
        let mut publisher = Publisher {
            stream: mock_tcpstream,
        };
        publisher.send_binary(b"hello test").unwrap();
        assert_eq!(&publisher.stream.write_data, b"message #hello test")
    }

    #[test]
    fn test_send_message() {
        let ok_message = b"ok message";
        let mut read_data = Vec::with_capacity(ok_message.len());
        read_data.extend_from_slice(ok_message);
        let mock_tcpstream = MockTcpStream {
            read_data,
            write_data: vec![],
        };
        let mut publisher = Publisher {
            stream: mock_tcpstream,
        };
        publisher.send_message(b"hello test").unwrap();
        assert_eq!(&publisher.stream.write_data, b"message \"hello test\"")
    }

    #[derive(Default)]
    struct MockTcpStream {
        write_data: Vec<u8>,
        read_data: Vec<u8>,
    }

    impl Write for MockTcpStream {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.write_data.extend(buf.iter());
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.write_data.clear();
            Ok(())
        }
    }

    impl Read for MockTcpStream {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let read_len = self.read_data.len();
            for i in 0..read_len {
                buf[i] = self.read_data[i];
            }
            Ok(read_len)
        }
    }
}
