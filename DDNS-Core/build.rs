use std::{env, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    validate_provider_feature();
    let crate_root = Path::new(&env::var("CARGO_MANIFEST_DIR")?).to_owned();
    let out_dir = crate_root.join("src/generated");
    let protoc = protoc_bin_vendored::protoc_bin_path().unwrap();
    unsafe {
        env::set_var("PROTOC", protoc);
    }
    let proto_include = protoc_bin_vendored::include_path().unwrap();
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir)?;
    }
    tonic_prost_build::configure()
        .out_dir(&out_dir)
        .build_client(true)
        .build_server(true)
        .compile_protos(&["./proto/ddns.proto"], &["./proto", proto_include.to_str().unwrap()])?;
    println!("cargo:rerun-if-changed=proto/ddns.proto");
    Ok(())
}
fn validate_provider_feature() {
    let count =
        env::vars().filter(|(k, v)| k.starts_with("CARGO_FEATURE_PROVIDER_") && v == "1").count();

    if count > 1 {
        panic!(
            "Only one provider feature may be enabled at a time. Prefix providers with \
             `provider-` and enable exactly one (e.g., `--features provider-cloudflare`)."
        );
    }
    // if count == 0 {
    //     println!("cargo:warning=No provider feature enabled. Build will
    // proceed without a concrete DNS provider."); }
}
