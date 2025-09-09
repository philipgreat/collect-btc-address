use bitcoin::{
    Address, Network, PrivateKey, PublicKey,
    secp256k1::{Secp256k1, SecretKey},
};
// use rand::RngCore;
// use rand::rngs::OsRng;

#[derive(Debug, Clone)]
pub struct KeyInfo {
    /// 私钥的 WIF 表示
    pub wif: String,
    /// P2PKH 地址
    pub p2pkh: String,
    /// P2WPKH 地址
    pub p2wpkh: String,
}

pub fn generate_address(data: &[u8]) -> KeyInfo {
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
        p2pkh: addr_p2pkh.to_string(),
        p2wpkh: addr_p2wpkh.to_string(),
    }
}
