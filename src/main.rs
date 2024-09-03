mod com;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    com::test()?;
    Ok(())
}
