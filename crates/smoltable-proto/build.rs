fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = tonic_prost_build::Config::new();
    let proto_dir = "./proto/";
    config.bytes([".table"]);
    let table_proto = format!("{}table.proto", proto_dir);

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_with_config(config, &[table_proto.clone()], &[proto_dir.to_string()])?;

    println!("cargo:rerun-if-changed={}", table_proto);
    Ok(())
}
