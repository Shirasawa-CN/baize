use anyhow::Result;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

async fn process(socket: TcpStream) -> Result<()> {
    let mut connection = Connection::new(socket);

    if let Some(_frame) = connection.read_frame().await.unwrap() {
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await?;
    }
    Ok(())
}

pub async fn tcp_stream() -> Result<()> {
    let listener = TcpListener::bind("localhost:8888").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket).await.unwrap();
        });
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_server_tcp_stream() {
        let _a = async move {
            use super::tcp_stream;
            let _test = tcp_stream();
            assert_eq!(_test.await.is_ok(), false);
        };
    }
}
