use backtrace::Backtrace;
use color_eyre::eyre;
use std::fmt::Write;
use tracing::{dispatcher::SetGlobalDefaultError, error, info};
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, Layer, Registry};


fn main() {
    println!("Hello, world!");
    let _ = set_logging();
    info!("HELLO WORLD!!!");
    error!("GOODBYE WORLD!!!");
}

pub fn set_logging() -> eyre::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        println!("Setting RUST_LOG env var to INFO");
        std::env::set_var("RUST_LOG", "info");
    }

    if std::env::var("CLOUD_LOGGING").is_ok() {
        println!("Setting gcl subscriber");
        custom_color_eyre_install()?;
        set_gcl_subscriber()?;
    } else {
        println!("Setting local subscriber");
        color_eyre::install()?;
        set_local_subscriber()?;
    }
    Ok(())
}

fn set_gcl_subscriber() -> Result<(), SetGlobalDefaultError> {
  tracing::subscriber::set_global_default(
    Registry::default().with(tracing_stackdriver::layer().with_filter(LevelFilter::INFO)),
  )
}

fn set_local_subscriber() -> Result<(), SetGlobalDefaultError> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt().finish())
}

fn custom_color_eyre_install() -> eyre::Result<()> {
    color_eyre::config::HookBuilder::new()
        .theme(color_eyre::config::Theme::new())
        .install()?;
    // Override the panic hook set by color_eyre.
    std::panic::set_hook(Box::new(|panic_info| {
        let mut msg = panic_info.to_string();
        if std::env::var("RUST_BACKTRACE").is_ok() {
            let backtrace = Backtrace::new();
            let _ = write!(msg, "\n{:━^80}\n{:?}", " BACKTRACE ", backtrace);
        }
        error!("{}", msg);
    }));
    Ok(())
}
