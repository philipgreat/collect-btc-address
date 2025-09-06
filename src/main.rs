use rand::RngCore;
use rand::rngs::OsRng;
mod KownAddressSet;
mod generate_address;
use KownAddressSet::StringSet;
fn main() {
    // 1) 随机生成私钥

    let mut rng = OsRng;
    let mut data = [0u8; 32];
    data[0] = 1; // 第一个字节为 1，其余都是 0
    //rng.fill_bytes(&mut data);
    println!("1");
    let string_set = StringSet::from_file("./address.list").expect("无法加载 address.list 文件");
    println!("Loaded {} entries", string_set.len());
    let count = 1_000_000_000;
    let report_count = 1_0_000_000;
    for i in 0..count {
        let keyinfo = generate_address::generate_address(&data);
        let test_words = vec![keyinfo.p2pkh, keyinfo.p2wpkh];
        for word in test_words {
            if string_set.contains(&word) {
                println!("address {} has key {} 在集合中", word, keyinfo.wif);
            }
        }
        if i % report_count == 0 {
            println!("finish {} times", i)
        }
        // TODO: 写入文件，而不是全存在内存里
        // 比如写 CSV / JSON
    }

    println!("finish {} times!", count);
    // let keyinfo = verify_by_address(&data);

    // println!("{:#?}", keyinfo);

    //5) 生成 P2TR (Taproot) 地址
    // let keypair = Keypair::from_secret_key(&secp, &secret_key);
    // let (xonly, _) = XOnlyPublicKey::from_keypair(&keypair);
    // let addr_p2tr = Address::p2tr(&secp, xonly, None, Network::Bitcoin);
    // println!("P2TR (Taproot) 地址: {}", addr_p2tr);
}
