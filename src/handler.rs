use mail_parser::MessageParser;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use tracing::{debug, error};

use crate::command::Command;

pub async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    debug!("Accepted connection from: {}", peer_addr);

    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer
        .write_all(b"220 Simple SMTP Server Ready\r\n")
        .await?;

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                debug!("Client disconnected: {}", peer_addr);
                break;
            }
            Ok(_) => {
                let command: Command = line.trim_end().to_string().into();
                debug!("Received: {:?}", command);

                match command {
                    Command::Helo(v) => {
                        writer
                            .write_all(b"250-devsmtp.com Hello, pleased to meet you\r\n250 AUTH GSSAPI DIGEST-MD5 PLAIN\r\n")
                            .await?
                    }
                    Command::Ehlo(v) => {
                        writer
                            .write_all(b"250 Hello, pleased to meet you\r\n")
                            .await?
                    }
                    Command::Auth(v) => {
                        writer
                            .write_all(b"235 2.7.0 Authentication successful\r\n")
                            .await?
                    }
                    Command::MailFrom(v) => {
                        writer.write_all(b"250 Sender OK\r\n").await?
                    }
                    Command::Receipient(v) => {
                        writer.write_all(b"250 Recipient OK\r\n").await?
                    }
                    Command::Data => {
                        writer
                            .write_all(b"354 Start mail input; end with <CRLF>.<CRLF>\r\n")
                            .await?;
                        let mut body = String::new();
                        loop {
                            let mut data = String::new();
                            reader.read_line(&mut data).await?;
                            if data.trim() == "." {
                                break;
                            }
                            body.push_str(&data);
                        }
                        let email = MessageParser::default().parse(body.as_bytes()).unwrap();

                        //debug!("{email:?}");

                        debug!("{}", serde_json::to_string(&email).unwrap());

                        writer.write_all(b"250 Message accepted\r\n").await?;
                    }
                    Command::Quit => {
                        writer.write_all(b"221 Bye\r\n").await?;
                        break;
                    }
                    _ => writer.write_all(b"500 Unrecognized command\r\n").await?,
                }
            }
            Err(e) => {
                error!("Error reading from client {}: {}", peer_addr, e);
                break;
            }
        }
    }

    Ok(())
}
