// use openssl::sha::sha256;
// use openssl::symm::{Cipher, Crypter, Mode};

#[cfg(test)]
mod test {
	use openssl::{
		sha::sha256,
		symm::{Cipher, Crypter, Mode},
	};

	#[test]
	fn testEncryption() {
		let plaintext = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".as_bytes();
		let key = sha256(b"This is a key");
		let IV = b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

		let encrypter =
			Crypter::new(Cipher::aes_256_gcm(), Mode::Encrypt, &key, Some(IV));
		let mut ciphertext = vec![0u8; 1024];
		let cipherlen = encrypter
			.unwrap()
			.update(plaintext, ciphertext.as_mut_slice())
			.unwrap();

		let decrypter =
			Crypter::new(Cipher::aes_256_gcm(), Mode::Decrypt, &key, Some(IV));
		let mut decrypted = vec![0u8; 1024];
		decrypter
			.unwrap()
			.update(&ciphertext[..cipherlen], decrypted.as_mut_slice())
			.unwrap();

		println!("{:?}", plaintext);
		println!("{:?}", ciphertext.as_slice());
		println!("{:?}", decrypted.as_slice());

		println!("{:?}", plaintext.len());
		println!("{:?}", ciphertext.len());
		println!("{:?}", decrypted.len());
	}
}
