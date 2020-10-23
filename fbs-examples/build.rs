use anyhow::Result;

fn main() -> Result<()> {
    for fbs in &[
        "fbs/array.fbs",
        "fbs/default_value.fbs",
        "fbs/greeter/greeter.fbs",
    ] {
        fbs_build::compile_fbs(fbs)?;
    }
    Ok(())
}
