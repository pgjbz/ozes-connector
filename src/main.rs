use ozes_connector::publisher::Publisher;

fn main() {
    let mut publisher = Publisher::builder()
        .with_host("localhost")
        .on_queue("foo")
        .build()
        .unwrap();
    publisher.send_message(b"Hello World!").unwrap();
    publisher.send_binary(b"foo").unwrap();
}
