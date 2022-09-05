fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/modelserver.proto")?;

    let generated =
        std::env::var("OUT_DIR")?.to_string() + "/modelserver.rs";
    std::fs::copy(&generated, "./src/modelserver.rs")?;
    // let x: &[&str] = &[];
    // tonic_build::configure()
    //     .out_dir("/Users/jh/projects/trade-sim/proto")
    //     .proto_path("/Users/jh/projects/trade-sim/proto")
    //     .compile(&["/Users/jh/projects/trade-sim/proto/helloworld.proto"], x)?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=path/to/Cargo.lock");

    Ok(())
}
