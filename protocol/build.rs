use protobuf_codegen::Codegen;

// Use this in build.rs
fn main() {
	Codegen::new()
		.includes(["src/proto"])
		.input("src/proto/messages.proto")
		.input("src/proto/network.proto")
		.cargo_out_dir("proto")
		.run_from_script();
}
