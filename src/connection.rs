use bytes::BytesMut;
use tokio::net::TcpStream;
use tokio::io::BufWriter;

use tokio::io::AsyncReadExt;
use bytes::Buf;

use mini_redis::{Frame, Result};
use mini_redis::frame::Error::Incomplete;
use std::io::Cursor;

pub struct Connection {
    // stream: TcpStream,
    stream: BufWriter<TcpStream>,
    // buffer: Vec<u8>,
    buffer: BytesMut,
    cursor: usize,
}



impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }
}
impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buffer: vec![0; 4096],
            cursor: 0,
        }
    }
    pub async fn read_frame(&mut self)-> Result<Option<Frame>>{
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // 确保缓冲区有容量
            if self.buffer.len() == self.cursor {
                // Grow the buffer
                self.buffer.resize(self.cursor * 2, 0);
            }

            // 读入缓冲区，跟踪桢的字节数
            let n = self.stream.read(&mut self.buffer[self.cursor..]).await?;
            
            if 0 == n {
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            } else {
                // 更新游标
                self.cursor += n;
            }
        }
    }



    fn parse_frame(&mut self)-> Result<Option<Frame>>{
        // Create the `T: Buf` type.
        let mut buf = Cursor::new(&self.buffer[..]);

        // 检查是否有完整的帧可用
        match Frame::check(&mut buf) {
            Ok(_) => {
                // 获取帧的字节长度
                let len = buf.position() as usize;

                // 重置内部游标用于调用 `parse` 方法
                buf.set_position(0);

                // 解析帧
                let frame = Frame::parse(&mut buf)?;

                // 从缓冲区中丢弃帧
                self.buffer.advance(len);

                // 将帧返回给调用方
                Ok(Some(frame))
            }
            // 缓冲区中的数据不足
            Err(Incomplete) => Ok(None),
            // 遇到错误
            Err(e) => Err(e.into()),
        }

    }

    use tokio::io::{self, AsyncWriteExt};

    async fn write_frame(&mut self, frame: &Frame)
        -> io::Result<()>
    {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => unimplemented!(),
        }

        self.stream.flush().await;

        Ok(())
    }

}
