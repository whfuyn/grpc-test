fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .format(true)
        .compile(
            &["test.proto"],
            &["proto"],
        )?;
    Ok(())
}
