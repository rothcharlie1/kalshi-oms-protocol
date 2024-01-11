use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {

    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");
    let (mut socket, _) = listener.accept().await.unwrap();

    loop {
        let _ = read_next_message(&mut socket).await;
        let wait = std::time::Duration::from_secs(5);
        std::thread::sleep(wait);
    }
}

async fn read_next_message(stream: &mut TcpStream) -> Result<Option<String>> {

    let mut len_buffer: [u8; 1] = [0; 1];
    let msg_len: usize = match stream.read(&mut len_buffer).await? {
        0 => return Ok(None),
        _ => usize::from(len_buffer[0]),
    }; 

    let mut type_buffer: [u8; 1] = [0; 1];
    match stream.read(&mut type_buffer).await? {
        0 => return Err(std::io::Error::other("Received length byte and no subsequent bytes").into()),
        _ => {
            let mut msg_content: Vec<u8> = vec![0; msg_len];
            stream.read(&mut msg_content).await?;
            match message_parser_factory(type_buffer[0]) {
                Some(f) => println!("{}", f(msg_len, &msg_content)?.unwrap_or(String::from("parser function provided no value"))),
                None => {
                    println!("found no function for this type");
                }
            }
        }
    }
    Ok(None)
}

fn message_parser_factory(msg_type: u8) -> Option<fn(usize, &[u8]) -> Result<Option<String>>> {
    match msg_type {
        5 => Some(parse_login_msg),
        _ => None
    }
}

fn parse_login_msg(length: usize, bytes: &[u8]) -> Result<Option<String>> {
    match length {
        10 => {
            let name_str = core::str::from_utf8(&bytes[..length])?;
            Ok(Some(name_str.to_string()))
        }
        _ => Err(std::io::Error::other("Provided login message with length not equal to 10").into())
    }
}
