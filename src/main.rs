use std::env;
use openssl::ec::{EcGroup, EcKey};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::rsa::Padding;
use openssl::sign::{Signer, Verifier};
use hex;

fn main() {
    // 获取传入的参数
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        show_help();
        return;
    }

    let mut generate_keys = false;
    let mut signature_requested = false;
    let mut verify_requested = false;

    let mut private_key: Option<String> = None;
    let mut message: Option<String> = None;
    let mut public_key: Option<String> = None;
    let mut signature: Option<String> = None;

    match args[1].as_str() {
        "-h" | "--help" => show_help(),
        "-g" | "--generate" => {
            generate_keys = true;
        }
        "-s" | "--signature" => {
            // 提取私钥和消息参数
            if 3 < args.len() {
                private_key = Some(args[2].clone());
                message = Some(args[3].clone());
                signature_requested = true;
            } else {
                println!("Invalid arguments for signature. Please provide private key and message.");
                show_help();
                return;
            }
        }
        "-v" | "--verify" => {
            // 提取公钥、消息、签名参数
            if 4 < args.len() {
                public_key = Some(args[2].clone());
                message = Some(args[3].clone());
                signature = Some(args[4].clone());
                verify_requested = true;
            } else {
                println!("Invalid arguments for verify. Please provide private key, message and signature.");
                show_help();
                return;
            }
        }
        _ => {
            println!("Invalid argument: {}", args[1]);
            show_help();
            return;
        }
    }
    if generate_keys {
        let (pr,pu) = generate_key_pair();
        println!("Generated key pair:");
        println!("Private Key (Hex): {}", pr);
        println!("Public Key (Hex): {}", pu);
    }

    if signature_requested {
        if let (Some(private_key), Some(message)) = (&private_key, &message) {
            let (signature, public_key) = signature_msg(private_key, message);
            println!("Signature: {}", signature);
            println!("Public Key: {}", public_key);
        } else {
            println!("Invalid arguments for signature. Please provide private key and message.");
            show_help();
        }
    }

    if verify_requested {
        if let (Some(public_key), Some(message), Some(signature)) = (&public_key, &message,&signature) {
            let result = verify_signature(&public_key, &message, &signature);
            println!("result: {}", result);
        } else {
            println!("Invalid arguments for signature. Please provide private key and message.");
            show_help();
        }
    }
}

//帮助
fn show_help() {
    println!("Usage: utility [options]");
    println!();
    println!("Options:");
    println!("  -h, --help        Display this help message");
    println!("  -g, --generate    Generate a key pair (private and public keys)");
    println!("  -s, --signature   <private_key_hex> <message>  Generate a signature");
    println!("  -v, --verify      <public_key_hex> <message> <signature_hex>  Verify a signature");
}


// 生成公私钥匙对
fn generate_key_pair() -> (String, String) {
    // 创建椭圆曲线群组，使用 NIST P-256 曲线
    let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).expect("Failed to create EC group");

    // 生成ec_key
    let ec_key = EcKey::generate(&group).unwrap();

    // 获取对应的公钥
    let public_key = ec_key.public_key_to_der().unwrap();
    let public_key_hex = hex::encode(&public_key);

    // 获取私钥
    let private_key = ec_key.private_key_to_der().unwrap();
    let private_key_hex = hex::encode(&private_key);

    (private_key_hex, public_key_hex)
}

// 16进制转bytes
fn hex_to_bytes(hex_str: &str) -> Vec<u8> {
    hex::decode(hex_str).expect("Failed to decode hex")
}

//签名方法
fn signature_msg(private_key_hex: &str, message: &str) -> (String, String) {
    println!("private_key_hex:{}",private_key_hex);
    println!("message:{}",message);

    // 将字符串转换为字节数组
    let message_bytes = message.as_bytes();

    // 将十六进制字符串解码为字节数组
    let private_key_bytes = hex_to_bytes(private_key_hex);

    // 转换私钥
    let private_key = PKey::private_key_from_der(&private_key_bytes).expect("Failed to create EC private key");

    // 创建签名器
    let mut signer = Signer::new(MessageDigest::sha256(), &private_key).unwrap();
    let _ = signer.set_rsa_padding(Padding::PKCS1);
    signer.update(message_bytes).unwrap();

    // 签名
    let signature_bytes = signer.sign_to_vec().unwrap();
    let signature = hex::encode(&signature_bytes);

    // 提取公钥
    let public_key_bytes = private_key.public_key_to_der().unwrap();
    let public_key = hex::encode(&public_key_bytes);

    (signature, public_key)
    
}

// 验证签名
fn verify_signature(public_key_hex: &str, message: &str, signature_hex: &str) -> bool {
    println!("public_key_hex:{}",public_key_hex);
    println!("message:{}",message);
    println!("signature_hex:{}",signature_hex);

    // 将十六进制字符串解码为字节数组
    let public_key_bytes = hex_to_bytes(public_key_hex);
    let signature_bytes = hex_to_bytes(signature_hex);

    // 获取公钥
    let public_key = PKey::public_key_from_der(&public_key_bytes).expect("Failed to create EC public key");

    // 创建验证器
    let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key).unwrap();
    let _ = verifier.set_rsa_padding(Padding::PKCS1);
    verifier.update(message.as_bytes()).unwrap();

    // 验签
    verifier.verify(&signature_bytes).unwrap()
}