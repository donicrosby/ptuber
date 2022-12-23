
use ptuber::{PTuber, Result};
fn main() -> Result<()> {
    let ptuber = PTuber::new();
    ptuber.start_ptuber()
}
