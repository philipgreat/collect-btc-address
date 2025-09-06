// use bitcoin::hashes::Hash;
// use bitcoin::hashes::{ripemd160, sha256}; // 使用 bitcoin 库中的哈希函数
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use bitcoin::util::key::PublicKey;
use rand::random;

use bitcoin::secp256k1::Secp256k1;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
//use secp256k1::{SecretKey};

fn generate_address_from_private_key(secret_key: &PrivateKey) -> String {
    // 通过私钥生成公钥
    let secp = Secp256k1::new();
    //let public_key = PublicKey::from_private_key(&secp, secret_key);
    let public_key = PublicKey::from_private_key(&secp, secret_key);

    // 获取公钥字节并计算公钥哈希
    //let pub_key_bytes = public_key.to_bytes();
    // let pub_key_hash = sha256::Hash::hash(&pub_key_bytes);
    // let pub_key_hash = ripemd160::Hash::hash(&pub_key_hash);

    // 生成 P2PKH 地址
    let network = Network::Bitcoin; // 使用主网
    let address = Address::p2pkh(&public_key, network);

    // 返回生成的地址
    address.to_string()
}

fn test_private_key(target_address: String, thread_id: usize, count: Arc<Mutex<u64>>) {
    let secp = Secp256k1::new();

    loop {
        // 生成随机私钥
        let secret_key = SecretKey::from_slice(&[random::<u8>(); 32]).unwrap();

        // 生成比特币地址
        let generated_address = generate_address_from_private_key(&secret_key);

        // 增加测试计数器
        let mut count_lock = count.lock().unwrap();
        *count_lock += 1;

        // 每100万次测试打印日志
        if *count_lock % 1_000_000 == 0 {
            println!("[Thread {}] Tested {} private keys", thread_id, count_lock);
        }

        // 检查生成的地址是否与目标地址匹配
        if generated_address == target_address {
            println!(
                "[Thread {}] Found matching private key! Address: {}",
                thread_id, generated_address
            );
            break;
        }
    }
}

fn main() {
    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <target_address> <num_threads>", args[0]);
        return;
    }

    let target_address = args[1].clone();
    let num_threads: usize = args[2].parse().expect("Invalid number of threads");

    let count = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    // 启动多线程
    for i in 0..num_threads {
        let target_address_clone = target_address.clone();
        let count_clone = Arc::clone(&count);

        let handle = thread::spawn(move || {
            test_private_key(target_address_clone, i, count_clone);
        });

        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
}
