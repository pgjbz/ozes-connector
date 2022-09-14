use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::errors::OzesResult;

pub type ConsumerClient = Consumer<TcpStream>;

pub struct Consumer<T>
where
    T: Read + Write,
{
    stream: T,
}

pub struct ConsumerBuilder<'s> {
    host: &'s str,
    port: u16,
    queue_name: &'s str,
    group_name: &'s str,
}

impl<'s> ConsumerBuilder<'s> {
    pub fn with_host(mut self, host: &'s str) -> Self {
        self.host = host;
        self
    }

    pub fn on_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn on_queue(mut self, queue_name: &'s str) -> Self {
        self.queue_name = queue_name;
        self
    }

    pub fn with_group(mut self, group_name: &'s str) -> Self {
        self.group_name = group_name;
        self
    }

    pub fn build(self) -> OzesResult<ConsumerClient> {
        Consumer::new(self)
    }
}

impl Default for ConsumerBuilder<'_> {
    fn default() -> Self {
        Self {
            host: "localhost",
            port: 7656,
            queue_name: "local_queue",
            group_name: "local_group",
        }
    }
}

impl Consumer<TcpStream> {
    pub fn builder<'s>() -> ConsumerBuilder<'s> {
        ConsumerBuilder::default()
    }

    fn new(builder: ConsumerBuilder) -> OzesResult<Self> {
        let mut stream = TcpStream::connect(format!("{}:{}", builder.host, builder.port))?;
        stream.write_all(
            format!(
                "subscribe {} with group {}",
                builder.queue_name, builder.queue_name
            )
            .as_bytes(),
        )?;
        crate::unwrap_return(&mut stream)?;

        Ok(Self { stream })
    }
}

impl<T> Consumer<T>
where
    T: Read + Write,
{
    pub fn read_message(&mut self) -> OzesResult<Vec<u8>> {
        let mut buffer = vec![0; 4096];
        match self.stream.read(&mut buffer) {
            Ok(n) => {
                buffer.truncate(n);
                let mut vec = Vec::with_capacity(8);
                vec.extend_from_slice(b"ok +l");
                vec.extend_from_slice(n.to_string().as_bytes());
                self.stream.write_all(&vec)?;
                Ok(buffer)
            }
            Err(err) => Err(err)?,
        }
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_read_message() {
        let ok_message = b"uou message";
        let mut read_data = Vec::with_capacity(ok_message.len());
        read_data.extend_from_slice(ok_message);
        let mock_tcpstream = MockTcpStream {
            write_calls: 0,
            read_data,
            write_data: vec![],
        };
        let mut consumer = Consumer {
            stream: mock_tcpstream,
        };
        consumer.read_message().unwrap();
        assert_eq!(
            &consumer.stream.write_data, b"ok +l11",
            "expected write ok message"
        );
        assert_eq!(
            &consumer.stream.read_data, b"uou message",
            "expected read \"uou message\""
        );
        assert_eq!(
            consumer.stream.write_calls, 1,
            "expected only one call to write"
        );
    }

    #[derive(Default)]
    struct MockTcpStream {
        write_data: Vec<u8>,
        write_calls: usize,
        read_data: Vec<u8>,
    }

    impl Write for MockTcpStream {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.write_calls += 1;
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
