use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&[
        "src/protos/data.proto",
        "src/protos/block.proto"
    ], &["src/"])?;
    Ok(())
}
