use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    // let workspace_dir = Path::new(&manifest_dir).parent().unwrap();
    dbg!("before compile");
    // tonic_build::compile_protos("../proto/types.proto")?;
    // let out_dir = workspace_dir.join("../proto_out");
    // let proto_path = workspace_dir.join("../proto");
    // dbg!(&out_dir);
    // dbg!(&proto_path);
    let proto_path: &Path = Path::new("../proto/types.proto");

    // directory the main .proto file resides in
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    tonic_build::configure()
        .out_dir("src")
        .compile(&[proto_path], &[proto_dir])?;

    // tonic_build::configure()
    //     .out_dir("proto_out")
    //     .compile(
    //         &["proto/types.proto"],
    //         &[],
    //     )?;
    dbg!("after compile");
    Ok(())
}