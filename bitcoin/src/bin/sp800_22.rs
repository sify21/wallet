use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    secp256k1::Secp256k1,
    Network,
};
use rand::{rngs::StdRng, RngCore, SeedableRng};
use snafu::{ResultExt, Whatever};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    str::FromStr,
};

pub fn main() -> Result<(), Whatever> {
    let dir = Path::new("/home/sify/Downloads/sp800-22-data");
    fs::create_dir_all(dir).with_whatever_context(|_| format!("can't create dir {dir:?}"))?;
    let secp = Secp256k1::new();
    let mut final_data_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&dir.join("data"))
        .with_whatever_context(|_| "can't open final data file")?;
    for i in 0..1000 {
        let data = dir.join(i.to_string());
        fs::create_dir_all(&data)
            .with_whatever_context(|_| format!("can't create sub dir {data:?}"))?;
        // 主私钥seed
        let mut rng = StdRng::from_entropy();
        let mut seed = [0u8; 16];
        rng.fill_bytes(&mut seed);
        // 保存seed
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&data.join("seed"))
            .with_whatever_context(|_| "can't open seed file")?;
        file.write_all(&seed)
            .with_whatever_context(|_| "can't write seed file")?;
        // 保存所有衍生私钥
        file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&data.join("data"))
            .with_whatever_context(|_| "can't open sub data file")?;
        // 主私钥
        let master = Xpriv::new_master(Network::Bitcoin, &seed)
            .with_whatever_context(|_| "can't create master xpriv")?;
        // 衍生私钥
        for idx in 0..100000 {
            let child = master
                .derive_priv(
                    &secp,
                    &DerivationPath::from_str(&format!("m/44'/0'/0'/0/{}", idx))
                        .with_whatever_context(|_| "derive path error")?,
                )
                .with_whatever_context(|_| "can't create child xpriv")?;
            //file.write_all(&child.private_key.secret_bytes())
            //.with_whatever_context(|_| "can't write sub data file")?;
            //final_data_file
            //.write_all(&child.private_key.secret_bytes())
            //.with_whatever_context(|_| "can't write final data file")?;
            // 丢弃首字节前缀
            file.write_all(&child.private_key.public_key(&secp).serialize()[1..])
                .with_whatever_context(|_| "can't write sub data file")?;
            final_data_file
                .write_all(&child.private_key.public_key(&secp).serialize()[1..])
                .with_whatever_context(|_| "can't write final data file")?;
        }
    }
    Ok(())
}
