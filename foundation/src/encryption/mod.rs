pub mod helpers;

use crate::prelude::TransformerFn;
use openssl::symm::{Cipher, Crypter, Mode};

#[allow(clippy::clone_on_copy)]
pub fn create_encryption_transformers(
	key: Vec<u8>,
	iv: &[u8; 32],
) -> (TransformerFn, TransformerFn) {
	// clone vecs
	let key1 = key.clone();
	let key2 = key.clone();

	let iv1 = iv.clone();
	let iv2 = iv.clone();

	(
		Box::new(move |plain_text| {
			println!("[encryptor_fn] plain_text:{:?}", plain_text);
			let encrypter = Crypter::new(Cipher::aes_256_gcm(), Mode::Encrypt, &key1, Some(&iv1));
			let mut ciphertext = vec![0u8; 128];
			let _cipherlen = encrypter
				.unwrap()
				.update(plain_text, &mut ciphertext)
				.unwrap();
			ciphertext
		}),
		Box::new(move |cipher_text| {
			println!("[decryptor_fn] cipher_text:{:?}", cipher_text);
			let decrypter = Crypter::new(Cipher::aes_256_gcm(), Mode::Decrypt, &key2, Some(&iv2));
			let mut plain_text = vec![0u8; 128];
			decrypter
				.unwrap()
				.update(cipher_text, &mut plain_text)
				.unwrap();
			plain_text
		}),
	)
}

#[cfg(test)]
mod test {
	use openssl::sha::sha256;
	use openssl::symm::{Cipher, Crypter, Mode};	
	
	use super::create_encryption_transformers;
	use super::helpers::create_test_shared;

	#[test]
	pub fn test_transformer_functions() {
		let shared = create_test_shared();

		let (en, de) = create_encryption_transformers(shared, b"12345678901234561234561234567765");

		let message = b"Hello world";

		let cipher_text = (*en)(message);

		assert_ne!(&cipher_text[0..message.len()], message);

		let decrypted_text = (*de)(&cipher_text);

		assert_eq!(&decrypted_text[0..message.len()], message);
	}

	#[test]
	pub fn test_aes() {
		let shared = create_test_shared();

		let plaintext = b"This is a message";
		let key = sha256(&shared);
		let iv = b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

		let encrypter = Crypter::new(Cipher::aes_256_gcm(), Mode::Encrypt, &key, Some(iv));
		let mut ciphertext = vec![0u8; 1024];
		let cipherlen = encrypter
			.unwrap()
			.update(plaintext, &mut ciphertext)
			.unwrap();

		let decrypter = Crypter::new(Cipher::aes_256_gcm(), Mode::Decrypt, &key, Some(iv));
		let mut decrypted = vec![0u8; 1024];
		decrypter
			.unwrap()
			.update(&ciphertext[..cipherlen], &mut decrypted)
			.unwrap();

		println!("plaintext: {:?}", plaintext);
		println!("ciphertext: {:?}", &ciphertext[0..plaintext.len()]);
		println!("decryptedtext: {:?}", &decrypted[0..plaintext.len()]);

		let test: &[u8] = &decrypted;

		assert_eq!(&test[0..plaintext.len()], plaintext);
	}
}
