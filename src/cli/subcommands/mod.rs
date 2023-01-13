mod subcommand;

mod help;
mod install;
mod list;
mod remove;
mod switch;
mod update;
mod version;

use self::{
    help::Help, install::Install, list::List, remove::Remove,
    switch::Switch, update::Update, version::Version,
};
use self::subcommand::Subcommand;

#[derive(clap::Subcommand)]
pub(crate) enum Subcommands {
    #[clap(about = "Prints the version")]
    Version(Version),

    #[clap(about = "Prints the help")]
    Help(Help),

    #[clap(about = "Install and switch to use the last version of FuckBF")]
    Update(Update),

    #[clap(about = "Install a specific version of FuckBF")]
    Install(Install),

    #[clap(about = "Remove a version of FuckBF")]
    Remove(Remove),

    // Uninstall is an alias for Remove
    #[clap(about = "Remove a version of FuckBF")]
    Uninstall(Remove),

    #[clap(about = "List all installed versions of FuckBF")]
    List(List),

    #[clap(about = "Switch to use a specific version of FuckBF")]
    Switch(Switch),

    // Alias for Switch
    #[clap(about = "Switch to use a specific version of FuckBF")]
    Use(Switch),
}
