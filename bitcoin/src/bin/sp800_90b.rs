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

fn main() -> Result<(), Whatever> {
    let dir = Path::new("/home/sify/Downloads/sp800-90b-data");
    fs::create_dir_all(dir).with_whatever_context(|_| format!("can't create dir {dir:?}"))?;
    let secp = Secp256k1::new();
    let mut rng = StdRng::from_entropy();
    // 生成一个主私钥，衍生1,000,000 输出sequential data，根据6.4，取前8位(第一个字节)
    let mut seed = [0u8; 16];
    rng.fill_bytes(&mut seed);
    let master = Xpriv::new_master(Network::Bitcoin, &seed)
        .with_whatever_context(|_| "can't create master xpriv")?;
    let mut sequential = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&dir.join("sequential-data"))
        .with_whatever_context(|_| "can't create sequential file")?;
    for idx in 0..1_000_000 {
        let child = master
            .derive_priv(
                &secp,
                &DerivationPath::from_str(&format!("m/44'/0'/0'/0/{}", idx))
                    .with_whatever_context(|_| "derive path error")?,
            )
            .with_whatever_context(|_| "can't derive child xpriv")?;
        sequential
            .write_all(&child.private_key.secret_bytes()[..1])
            .with_whatever_context(|_| "can't write sequential")?;
    }
    // 生成1000*1000的矩阵用于重启测试
    let mut matrix: Vec<Vec<Xpriv>> = vec![];
    for _ in 0..1000 {
        let mut seed = [0u8; 16];
        rng.fill_bytes(&mut seed);
        let master = Xpriv::new_master(Network::Bitcoin, &seed)
            .with_whatever_context(|_| "can't creaet matrix master xpriv")?;
        // 衍生私钥
        let mut xprivs = vec![];
        for idx in 0..1000 {
            xprivs.push(
                master
                    .derive_priv(
                        &secp,
                        &DerivationPath::from_str(&format!("m/44'/0'/0'/0/{}", idx))
                            .with_whatever_context(|_| "matrix derive path error")?,
                    )
                    .with_whatever_context(|_| "can't derive matrix xpriv")?,
            );
        }
        matrix.push(xprivs);
    }
    let mut row = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&dir.join("row-data"))
        .with_whatever_context(|_| "can't open row-data file")?;
    let mut column = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&dir.join("column-data"))
        .with_whatever_context(|_| "can't open column-data file")?;
    for i in 0..1000 {
        for j in 0..1000 {
            // 生成row dataset
            row.write_all(&matrix[i][j].private_key.secret_bytes()[..1])
                .with_whatever_context(|_| "can't write row-data file")?;
            // 生成column dataset
            column
                .write_all(&matrix[j][i].private_key.secret_bytes()[..1])
                .with_whatever_context(|_| "can't write column-data file")?;
        }
    }
    Ok(())
}
