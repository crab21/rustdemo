use tokio::{
    fs::File,
    io::{self, AsyncReadExt},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = String::from("/Users/gogoowang/learn/rustdemo/src/tokio_ln/mod.rs");
    let mut f = File::open(path).await?;
    let mut buf = [0; 10];
    let n = f.read(&mut buf[..]).await?;

    println!(
        "{:?}",
        String::from_utf8(buf[..n].to_vec()).unwrap_or_default()
    );
    Ok(())
}

#[test]
fn test_tokio() {
    let a = main();
    println!("{:?}", a)
}
