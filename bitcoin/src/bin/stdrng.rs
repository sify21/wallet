use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use rand::{distributions::Standard, rngs::StdRng, Rng, SeedableRng};
use snafu::{ResultExt, Whatever};

pub fn main() -> Result<(), Whatever> {
    let dir = Path::new("/home/sify/Downloads/sp800-22-data");
    fs::create_dir_all(dir).with_whatever_context(|_| format!("can't create dir {dir:?}"))?;
    let mut final_data_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&dir.join("data"))
        .with_whatever_context(|_| "can't open final data file")?;
    for _ in 0..1000 {
        let mut rng = StdRng::from_entropy();
        for _ in 0..100000 {
            let bytes: [u8; 32] = rng.sample(Standard);
            final_data_file
                .write_all(&bytes)
                .with_whatever_context(|_| "can't write final data file")?;
        }
    }
    Ok(())
}
