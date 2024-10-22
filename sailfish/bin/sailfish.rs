use sailfish::prelude::*;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runner = Runner::new()?;

    sailfish::init_logger();

    runner.block_after_init(move |factory| {
        let handler = factory.spawn_handle();

        handler.spawn("binance", move || async move {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            tracing::info!("todo");
            ok()
        });

        Ok(())
    })?;

    Ok(())
}
