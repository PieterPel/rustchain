use openssl::encrypt::Encrypter;
use openssl::pkey::PKey;
use openssl::pkey::PKeyRef;
use openssl::pkey::{Private, Public};
use openssl::rsa::{Padding, Rsa};

use crate::utils::StringBytes;
use crate::BlockChain;

const PAIR_SIZE_IN_BITS: u32 = 2048;

struct Miner;

pub struct Person {
    pub public_key: Vec<u8>,
    rsa_certificate: Rsa<Private>,
}

impl Person {
    pub fn new() -> Person {
        let (rsa_certificate, public_key) = generate_key_pair();

        Person {
            public_key,
            rsa_certificate,
        }
    }

    fn encrypt_message(&self, message: &str, other_public_key_pem: &[u8]) -> Vec<u8> {
        let message_bytes = message.as_bytes();

        // Convert the public key bytes to PKey<Public>
        let other_public_key = PKey::public_key_from_pem(other_public_key_pem)
            .expect("Failed to create PKey from public key bytes");

        // Encrypt message using the other public key
        let mut encrypter = Encrypter::new(&other_public_key).unwrap();
        encrypter.set_rsa_padding(Padding::PKCS1).unwrap();

        let buffer_len = encrypter.encrypt_len(message_bytes).unwrap();

        let mut encrypted = vec![0; buffer_len];

        encrypter.encrypt(message_bytes, &mut encrypted).unwrap();

        encrypted
    }

    pub fn send_message(
        self,
        message: &str,
        other_public_key: &[u8],
        block_chain: &mut BlockChain<StringBytes>,
    ) {
        let encrypted_message = self.encrypt_message(message, other_public_key);

        let mut nonce: u64 = 1;
        loop {
            let data = StringBytes {
                bytes: encrypted_message.clone(),
            };

            let new_block = block_chain.new_block(data, nonce);

            match block_chain.try_adding_block(new_block) {
                Ok(()) => {
                    // Our nonce gives a good hash!
                    println!(
                        "Nonce {} gives good hash: {}",
                        nonce,
                        block_chain.last_hash()
                    );
                    break;
                }
                Err(_error) => {
                    // println!("Nonce {} gives a bad hash: {}", nonce, _error);
                }
            }

            nonce += 1;
        }
    }
}

fn generate_key_pair() -> (Rsa<Private>, Vec<u8>) {
    // Generate an RSA key pair with a specified number of bits
    let rsa_certificate =
        Rsa::generate(PAIR_SIZE_IN_BITS).expect("Failed to generate RSA key pair");

    // Extract the public key
    let public_key = PKey::from_rsa(rsa_certificate.clone())
        .expect("Failed to create Public Key from RSA certificate")
        .public_key_to_pem()
        .expect("Failed to convert public key to PEM format");

    // Create a PKey object from the generated RSA key
    //let private_key = PKey::from_rsa(rsa).expect("Failed to create PKey from RSA key");

    (rsa_certificate, public_key)
}
