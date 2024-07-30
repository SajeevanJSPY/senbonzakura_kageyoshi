use crate::cli::{Cli, Subcommand};
use sc_cli::SubstrateCli;

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Senbonzakura Kageyoshi".into()
    }

    fn impl_version() -> String {
        "0.0.1".into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "github.com/sajeevanjspy/senbonzakura_kageyoshi".into()
    }

    fn copyright_start_year() -> i32 {
        2024
    }

    fn load_spec(&self, _id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        unimplemented!()
    }
}

pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        _ => todo!(),
    }
}
