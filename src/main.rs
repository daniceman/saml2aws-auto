extern crate chrono;
#[macro_use]
extern crate clap;
extern crate crossterm;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate serde_yaml;

extern crate base64;
extern crate cookie;
extern crate dirs;
extern crate ini;
extern crate keyring;
extern crate openssl_probe;
extern crate reqwest;
extern crate rpassword;
extern crate scraper;

mod aws;
mod config;
mod groups;
mod keycloak;
mod refresh;
mod saml;
mod update;

use std::env;
use std::io;

use clap::App;
use crossterm::style::{style, Color};
use crossterm::Screen;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);

    // Check for a new version
    if let Ok(update::VersionComparison::IsDifferent) =
        update::compare_version(&env::var("CARGO_PKG_VERSION").unwrap())
    {
        let screen = Screen::default();

        println!(
            "\n\t{}",
            style("A new version of saml2aws-auto is available")
                .with(Color::Green)
                .into_displayable(&screen)
        );
        println!("\tIf you want to enjoy the greatest and latest features, make sure to update\n\tyour installation of saml2aws-auto.");
        println!("");
    };

    let matches = app.get_matches();

    if let Some(_) = matches.subcommand_matches("version") {
        App::from_yaml(yaml)
            .write_long_version(&mut io::stdout())
            .unwrap();
        return;
    }

    let verbosity = matches.occurrences_of("verbose");

    if !config::check_or_interactive_create() {
        return;
    }

    if let Some(matches) = matches.subcommand_matches("groups") {
        groups::command(matches)
    } else if let Some(_) = matches.subcommand_matches("configure") {
        let cfg = config::load_or_default()
            .expect("Internal error when trying to read config. Please open an issue on GitHub.");
        config::interactive_create(cfg)
    } else if let Some(matches) = matches.subcommand_matches("refresh") {
        refresh::command(matches, verbosity)
    }
}
