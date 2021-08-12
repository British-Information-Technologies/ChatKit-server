use openssl::derive::Deriver;
use openssl::ec::EcGroup;
use openssl::ec::EcKey;
use openssl::nid::Nid;
use openssl::pkey::PKey;

pub fn create_test_shared() -> Vec<u8> {
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