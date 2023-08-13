use rsa::RsaPrivateKey;
//use rsa::traits::PublicKeyParts;
//use rsa::RsaPublicKey;
//use rsa::traits::PaddingScheme;
use rsa::pkcs1v15;
//use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // 키 생성
    let bits = 1024;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("키 생성 실패");
    let public_key = private_key.to_public_key();

    // 암호화할 데이터
    let data = b"what is love we can fly~";

    // 데이터 암호화
    //let encrypted_data = vec![0; public_key.size() as usize];
    let a = public_key.encrypt(&mut rng, pkcs1v15::Pkcs1v15Encrypt, data)
        .expect("데이터 암호화 실패");
    

    // 데이터 복호화
    //let decrypted_data = vec![0; private_key.size() as usize];
    let b = private_key.decrypt(pkcs1v15::Pkcs1v15Encrypt, &a)
        .expect("데이터 복호화 실패");

    println!("원본: {:?}", String::from_utf8_lossy(data));
    println!("암호화된 데이터: {:?}", a);
    println!("복호화된 데이터: {:?}", String::from_utf8_lossy(&b));
    //println!("key : {:?}", private_key)
}