use std::io::Result;

// Use this in build.rs
fn main() -> Result<()> {
	prost_build::compile_protos(
		&["src/proto/network.proto", "src/proto/connected.proto"],
		&["src/proto"],
	)?;
	Ok(())
}
