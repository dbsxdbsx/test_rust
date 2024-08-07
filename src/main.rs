mod test_windows_command_runner;
use anyhow::Result;

fn main() -> Result<()> {
    test_windows_command_runner::test(7)?;
    Ok(())
}
