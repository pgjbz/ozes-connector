#[derive(Debug)]
pub enum Response {
    Ok,
    Err { message: Vec<u8> },
}
