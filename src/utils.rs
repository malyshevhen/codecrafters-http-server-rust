use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use crate::error::Result;

pub async fn read_file_content(directory_path: &str) -> Result<String> {
    println!("{:<12} - read_file_content", "UTILS");
    
    let mut file = File::open(directory_path).await?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).await?;

    Ok(file_content)
}

pub async fn write_file(full_path: &str, file_content: &str) -> Result<()> {
    println!("{:<12} - write_file", "UTILS");
   
    let mut file = File::create(full_path).await?;
    file.write_all(file_content.as_bytes()).await?;
    Ok(())
}
