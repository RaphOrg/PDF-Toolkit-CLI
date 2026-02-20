mod app;
mod cli;
mod commands;

fn main() -> anyhow::Result<()> {
    app::run()
}
