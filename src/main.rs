use std::{
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::{Parser, Subcommand};
use nix_api::{install_package, remove_package};

mod nix_api;

#[derive(Parser)]
#[command(name = "PIE-NIX")]
#[command(about = "A CLI tool to manage NIX packages on EPITA NIX hardware", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install one or more packages
    Install {
        /// A list of packages to install
        #[arg()]
        packages: Vec<String>,

        /// If specified, the aforementioned packages will not be auto-logged, and will not be reinstalled next boot
        #[arg(short, long)]
        temp: bool,
    },

    /// Uninstall one or more packages
    Remove {
        /// A list of packages to remove
        #[arg()]
        packages: Vec<String>,

        /// If specified, this removal will not be auto-logged, and the packages will be
        /// reinstalled next boot
        #[arg(short, long)]
        temp: bool,
    },

    /// Reinstall packages at boot
    Recover {},

    /// List managed packages
    ListPackages {},

    SetInstallOrder {},
}

fn main() {
    // Get path to home $HOME
    let home_string = std::env::var("HOME")
        .expect("Home directory unset. Are you running pie-nix from the right user ?");

    let home_dir = PathBuf::from_str(&home_string).expect("Error opening HOME directory");

    //Parse config directory
    let config_directory: PathBuf = Path::join(&home_dir, ".pie-nix");
    let pkgs_file: PathBuf = Path::join(&config_directory, "pkgs.pnix");

    //Every string is a package name.
    let pkgs_data: Vec<String>;

    if !config_directory.exists() {
        match std::fs::create_dir_all(&config_directory) {
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Ok(_) => {}
            Err(_) => {
                panic!(
                    "Couldn't create a configuration directory at {:?}. Aborting...",
                    config_directory
                )
            }
        }
    }

    // List of packages to install
    pkgs_data = match std::fs::read_to_string(pkgs_file) {
        Err(_) => vec![],
        Ok(s) => s.lines().map(|x| String::from(x)).collect(),
    };

    let cli = Cli::parse();
    match &cli.command {
        Commands::Install { packages, .. } => {
            for package in packages{
                if install_package(package) {
                    println!("Installed {}", package);
                } else {
                    println!("Failed to install {}", package);
                }
            }
        }
        Commands::Remove { packages, .. } => {
            println!("Trying to remove {:?}", packages);
        }
        Commands::Recover {} => {
            println!("I am reinstalling the packages");
            todo!("Parse the config folder, and reinstall every package in it");
        }
        Commands::ListPackages {} => {
            todo!("List packages here")
        }

        Commands::SetInstallOrder {} => {
            todo!("Menu to config in which order to install")
        }
    }
}
