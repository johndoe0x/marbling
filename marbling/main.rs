use clap::{App, AppSettings};

#[cfg_attr(tarpaulin, skip)]

fn main() {
    let app_version = "v0.0.1";
    let app_description = "Crypto wallet which supports dkg, mpc signing and account recovery.";
    let app_dev = "johndoe0x@proton.mail";
    let app_setting = &[
        AppSettings::ColoredHelp,
        AppSettings::DisableHelpSubcommand,
        AppSettings::DisableVersion,
        AppSettings::SubcommandRequiredElseHelp,
    ];
    let app_subcommands = vec![
        //NormalwalletCLI::new();
        //MPCwalletCLI::new();
    ];
    let arguments = App::new("Marbling")
        .version(app_version)
        .about(app_description)
        .author(app_dev)
        .settings(app_setting)
        .subcommands(app_subcommands)
        .set_term_width(0)
        .get_matches();
    
    match arguments.subcommand() {
        //TODO : start command working as wallet server daemon
        //("start")
        ()
    }
}