use bitcoin::{
    Address, Network, PrivateKey, PublicKey,
    secp256k1::{Secp256k1, SecretKey},
};
use rand::RngCore;
use rand::rngs::OsRng;

#[derive(Debug, Clone)]
pub struct KeyInfo {
    /// 私钥的 WIF 表示
    pub wif: String,
    /// 公钥（hex 压缩形式）
    pub public_key: String,
    /// P2PKH 地址
    pub p2pkh: String,
    /// P2WPKH 地址
    pub p2wpkh: String,
}

fn verify_by_address(data: &[u8]) -> KeyInfo {
    let secp = Secp256k1::new();

    let secret_key = SecretKey::from_slice(&data).expect("32 bytes, within curve order");
    //let private_key = PrivateKey::new(secret_key, Network::Bitcoin);

    let private_key = PrivateKey {
        network: Network::Bitcoin,
        compressed: true,
        inner: secret_key,
    };

    // 2) 生成公钥
    let public_key = PublicKey::from_private_key(&secp, &private_key);

    // 3) 生成 P2PKH 地址
    let addr_p2pkh = Address::p2pkh(&public_key, Network::Bitcoin);

    //4) 生成 P2WPKH (bech32) 地址

    let addr_p2wpkh = Address::p2wpkh(&public_key, Network::Bitcoin).expect("需要压缩公钥");

    KeyInfo {
        wif: private_key.to_wif(),
        public_key: public_key.to_string(),
        p2pkh: addr_p2pkh.to_string(),
        p2wpkh: addr_p2wpkh.to_string(),
    }
}

fn main() {
    // 1) 随机生成私钥

    let mut rng = OsRng;
    let mut data = [0u8; 32];
    data[0] = 1; // 第一个字节为 1，其余都是 0
    //rng.fill_bytes(&mut data);
    println!("1");
    for i in 0..1_000_000 {
        let keyinfo = verify_by_address(&data);

        // 打印前 10 个，其余不打印
        if i < 10 {
            println!("{:?}", keyinfo);
        }

        // TODO: 写入文件，而不是全存在内存里
        // 比如写 CSV / JSON
    }
    // let keyinfo = verify_by_address(&data);

    // println!("{:#?}", keyinfo);

    //5) 生成 P2TR (Taproot) 地址
    // let keypair = Keypair::from_secret_key(&secp, &secret_key);
    // let (xonly, _) = XOnlyPublicKey::from_keypair(&keypair);
    // let addr_p2tr = Address::p2tr(&secp, xonly, None, Network::Bitcoin);
    // println!("P2TR (Taproot) 地址: {}", addr_p2tr);
}
