


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

	#[test]
	pub fn test_aes() {

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

		let shared1 = deriver1.derive_to_vec().unwrap();
		let shared2 = deriver2.derive_to_vec().unwrap();

		println!("shared1: {:?}", &shared1);
		println!("shared2: {:?}", &shared2);

		assert_eq!(shared1, shared2);
		
		let plaintext = b"This is a message";
		let key = sha256(&shared1);
		let iv = b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

		let encrypter = Crypter::new(Cipher::aes_256_gcm(), Mode::Encrypt, &key, Some(iv));
		let mut ciphertext = vec![0u8; 1024];
		let cipherlen = encrypter.unwrap().update(plaintext, ciphertext.as_mut_slice()).unwrap();

		let decrypter = Crypter::new(Cipher::aes_256_gcm(), Mode::Decrypt, &key, Some(iv));
		let mut decrypted = vec![0u8; 1024];
		decrypter.unwrap().update(&ciphertext[..cipherlen], decrypted.as_mut_slice()).unwrap();

		println!("plaintext: {:?}", plaintext);
		println!("ciphertext: {:?}", &ciphertext[0..plaintext.len()]);
		println!("decryptedtext: {:?}", &decrypted[0..plaintext.len()]);
	}

}