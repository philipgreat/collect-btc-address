// use bitcoin::key;
use rand::RngCore;
use rand::rngs::OsRng;
mod generate_address;
mod kown_address_set;
use kown_address_set::StringSet;
use tracing::{info};

fn main() {
    // 1) 随机生成私钥

    let mut rng = OsRng;

    tracing_subscriber::fmt::init();
    info!("started");
    let string_set = StringSet::from_file("./address.list").expect("无法加载 address.list 文件");
    info!("Loaded {} entries", string_set.len());
    let count = 1_000_000_000;
    //let report_count = 1_0_000_000;
    for _ in 0..count {
        let mut data = [0u8; 32];
        rng.fill_bytes(&mut data);
        let keyinfo = generate_address::generate_address(&data);
        let test_words = vec![keyinfo.p2pkh, keyinfo.p2wpkh];
        //info!("verify for key {} with public address ", keyinfo.wif);

        for word in test_words {
            if string_set.contains(&word) {
                println!("address {} has key {} 在集合中", word, keyinfo.wif);
            }
        }
    }

    info!("finish {} times!", count);
    // let keyinfo = verify_by_address(&data);

    // println!("{:#?}", keyinfo);

    //5) 生成 P2TR (Taproot) 地址
    // let keypair = Keypair::from_secret_key(&secp, &secret_key);
    // let (xonly, _) = XOnlyPublicKey::from_keypair(&keypair);
    // let addr_p2tr = Address::p2tr(&secp, xonly, None, Network::Bitcoin);
    // println!("P2TR (Taproot) 地址: {}", addr_p2tr);
}
