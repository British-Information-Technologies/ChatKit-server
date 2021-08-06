use openssl::symm::{
	Cipher,
	Crypter,
	Mode
};

type TransformerFn = dyn Fn(&[u8]) -> Vec<u8>;

#[allow(clippy::clone_on_copy)]
pub fn create_encryption_transformers(key: Vec<u8>, iv: &[u8; 32])
	-> (Box<TransformerFn>,Box<TransformerFn>)
{
	// clone vecs
	let key1 = key.clone();
	let key2 = key.clone();

	let iv1 = iv.clone();
	let iv2 = iv.clone();

	(
		Box::new(move |plain_text| {
			let encrypter = Crypter::new(Cipher::aes_256_gcm(), Mode::Encrypt, &key1, Some(&iv1));
			let mut ciphertext = vec![0u8; 1024];
			let _cipherlen = encrypter.unwrap().update(plain_text, &mut ciphertext).unwrap();
			ciphertext
		}),
		Box::new(move |cipher_text| {
			let decrypter = Crypter::new(Cipher::aes_256_gcm(), Mode::Decrypt, &key2, Some(&iv2));
			let mut plain_text = vec![0u8; 1024];
			decrypter.unwrap().update(cipher_text, &mut plain_text).unwrap();
			plain_text
		})
	)
}

#[cfg(test)]
mod test {
	use openssl::ec::*;
	use openssl::nid::Nid;
	use openssl::ec::EcKey;
	use openssl::pkey::PKey;
	use openssl::sha::sha256;
	use openssl::derive::Deriver;
	use openssl::symm::{
		Cipher,
		Crypter,
		Mode
	};

	use super::create_encryption_transformers;

	fn create_shared() -> Vec<u8> {
		let ec_group1 = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
		let ec_group2 = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();

		let eckey1 = EcKey::generate(ec_group1.as_ref()).unwrap();
		let eckey2 = EcKey::generate(ec_group2.as_ref()).unwrap();
			
		let pkey1 = PKey::from_ec_key(eckey1).unwrap();
		let pkey2 = PKey::from_ec_key(eckey2).unwrap();

		let pem1 = pkey1.public_key_to_pem().unwrap();
		let pem2 = pkey2.public_key_to_pem().unwrap();

		let pub1 = PKey::public_key_from_pem(&pem1).unwrap();
		let pub2 = PKey::public_key_from_pem(&pem2).unwrap();

		let mut deriver1 = Deriver::new(pkey1.as_ref()).expect("deriver1 failed");
		let mut deriver2 = Deriver::new(pkey2.as_ref()).expect("deriver2 failed");

		deriver1.set_peer(pub2.as_ref()).unwrap();
		deriver2.set_peer(pub1.as_ref()).unwrap();

		deriver1.derive_to_vec().unwrap()
	}

	#[test]
	pub fn test_transformer_functions() {
		let shared = create_shared();

		let (en, de) = create_encryption_transformers(shared, b"12345678901234561234561234567765");

		let message = b"Hello world";

		let cipher_text = (*en)(message);
		let decrypted_text = (*de)(&cipher_text);

		assert_eq!(&decrypted_text[0..message.len()], message);
	}

	#[test]
	pub fn test_aes() {

		let shared = create_shared();
		
		let plaintext = b"This is a message";
		let key = sha256(&shared);
		let iv = b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

		let encrypter = Crypter::new(Cipher::aes_256_gcm(), Mode::Encrypt, &key, Some(iv));
		let mut ciphertext = vec![0u8; 1024];
		let cipherlen = encrypter.unwrap().update(plaintext, &mut ciphertext).unwrap();

		let decrypter = Crypter::new(Cipher::aes_256_gcm(), Mode::Decrypt, &key, Some(iv));
		let mut decrypted = vec![0u8; 1024];
		decrypter.unwrap().update(&ciphertext[..cipherlen], &mut decrypted).unwrap();

		println!("plaintext: {:?}", plaintext);
		println!("ciphertext: {:?}", &ciphertext[0..plaintext.len()]);
		println!("decryptedtext: {:?}", &decrypted[0..plaintext.len()]);

		let test: &[u8] = &decrypted;

		assert_eq!(&test[0..plaintext.len()], plaintext);
	}
}