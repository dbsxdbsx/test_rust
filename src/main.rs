mod test_ip;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    test_ip::test().await?;
    Ok(())
}
