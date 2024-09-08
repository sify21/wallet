use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use rand::{distributions::Standard, rngs::StdRng, Rng, RngCore, SeedableRng};
use snafu::{ResultExt, Whatever};

pub fn main() -> Result<(), Whatever> {
    //let dir = Path::new("/home/sify/Downloads/sp800-22-data");
    //fs::create_dir_all(dir).with_whatever_context(|_| format!("can't create dir {dir:?}"))?;
    //let mut final_data_file = OpenOptions::new()
    //.create(true)
    //.truncate(true)
    //.write(true)
    //.open(&dir.join("data"))
    //.with_whatever_context(|_| "can't open final data file")?;
    //for _ in 0..1000 {
    //let mut rng = StdRng::from_entropy();
    //for _ in 0..100000 {
    //let bytes: [u8; 32] = rng.sample(Standard);
    //final_data_file
    //.write_all(&bytes)
    //.with_whatever_context(|_| "can't write final data file")?;
    //}
    //}
    let dir = Path::new("/home/sify/Downloads/sp800-90b-data");
    fs::create_dir_all(dir).with_whatever_context(|_| format!("can't create dir {dir:?}"))?;
    let mut sequential_data_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&dir.join("sequential-data"))
        .with_whatever_context(|_| "can't open sequential data file")?;
    let mut row_data_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&dir.join("row-data"))
        .with_whatever_context(|_| "can't open row data file")?;
    let mut rng = StdRng::from_entropy();
    let mut d = [0u8; 1_000_000];
    rng.fill_bytes(&mut d);
    sequential_data_file
        .write_all(&d)
        .with_whatever_context(|_| "can't write sequential")?;
    for _ in 0..1000 {
        let mut rng = StdRng::from_entropy();
        let mut row_data = [0u8; 1000];
        rng.fill_bytes(&mut row_data);
        row_data_file
            .write_all(&row_data)
            .with_whatever_context(|_| "can't write row-data file")?;
    }
    Ok(())
}
