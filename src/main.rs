use wgpu_test::run;

fn main() -> anyhow::Result<()> {
    smol::block_on(run())?;
    Ok(())
}
