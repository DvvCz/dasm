mod tier;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("cargo:rerun-if-changed=build/main.rs");

	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

	std::fs::write(out_path.join("x86.rs"), tier::x86::src())?;
	std::fs::write(out_path.join("amd64.rs"), tier::amd64::src())?;

	Ok(())
}
