use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // let mut f = File::open("foo.txt").await?;
    // let mut buffer = [0; 10];

    // // read up to 10 bytes
    // let n = f.read(&mut buffer[..]).await?;

    // println!("The bytes: {:?};size={:?}", &buffer[..n], n);
    // Ok(())

    // ------------------------------------------------

    // let mut f = File::open("foo.txt").await?;
    // let mut buffer = Vec::new();

    // // read the whole file
    // f.read_to_end(&mut buffer).await?;
    // let len = buffer.len();
    // println!("The bytes: {:?};size={:?}", &buffer,len);
    // Ok(())

    // ------------------------------------------------
    // let mut file = File::create("foo.txt").await?;

    // // Writes some prefix of the byte string, but not necessarily all of it.
    // let n = file.write(b"some bytes").await?;

    // println!("Wrote the first {} bytes of 'some bytes'.", n);
    // Ok(())
    // ------------------------------------------------

    // let mut file = File::create("foo.txt").await?;

    // file.write_all(b"some bytes").await?;
    // Ok(())

    // ------------------------------------------------

    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}