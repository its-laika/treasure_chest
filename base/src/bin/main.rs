use rusty_box_base::base64;
use rusty_box_base::encryption::{Encoding, Encryption, XChaCha20Poly1305};
use rusty_box_base::file::{get_random_file_path, load, store};
use rusty_box_base::hash::{Argon2, Hashing};

fn main() {
    let message = "Hello World!";
    let file_path = get_random_file_path().unwrap();

    let (data, key) = XChaCha20Poly1305::encrypt(message.as_bytes()).unwrap();

    let content = data.encode();
    store(&file_path, &content).unwrap();

    let file_content = load(&file_path).unwrap();
    let file_encryption_data = XChaCha20Poly1305::decode(&file_content).unwrap();
    let binding = file_encryption_data.decrypt(&key).unwrap();
    let decrypted = std::str::from_utf8(&binding).unwrap();

    println!("Plain data: {}", message);
    println!("Encrypted file: {}", &file_path.display());
    println!("Decrypted data: {}", decrypted);

    if decrypted == message {
        println!("Decrypted matches plain!")
    }

    let key_hash = Argon2::hash(&key).unwrap();
    println!("Key (base64): {}", base64::encode(&key));
    println!("Key (argon2): {}", &key_hash);

    if Argon2::verify(&key, &key_hash).unwrap() {
        println!("Hash matches key.");
    } else {
        println!("Hash mismatch!")
    }
}
