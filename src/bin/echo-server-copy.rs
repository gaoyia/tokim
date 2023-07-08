use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6379").await?;
    let (mut rd, mut wr) = io::split(socket);

    // 在后台写数据
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;

        // // 可选：刷新流，确保消息已发送
        // wr.flush().await?;

        wr.write_all(b"world\r\n").await?;
        
        // // 可选：关闭连接
        // drop(wr);


        // 有时，Rust类型推断器需要一点帮助。
        Ok::<_, io::Error>(())

        
    });

    let mut buf = vec![0; 8192];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}