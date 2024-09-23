use std::{
    io,
    path::{Path, PathBuf},
    str::FromStr,
};
use std::io::stdout;
use std::process::exit;
use clap::{Parser, Subcommand};
use crossterm::cursor::{MoveDown, MoveUp};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use serde::{Deserialize, Serialize};
use crate::nix_api::{install_package, remove_package, require_nix};

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
    Reload {},

    /// List managed packages
    ListPackages {},

    // Choose in which order pie-nix reload will install the packages
    //SetInstallOrder {},
}

#[derive(Serialize, Deserialize)]
struct SavedData {
    packages: Vec<String>
}

fn main() {
    // Get path to home $HOME
    let home_string = std::env::var("HOME")
        .expect("Home directory unset. Are you running pie-nix from the right user ?");

    let home_dir = PathBuf::from_str(&home_string).expect("Error opening HOME directory");
    let afs_dir = Path::join(&home_dir, "afs");

    //Parse config directory
    let config_directory: PathBuf = Path::join(&afs_dir, ".pie-nix");
    let pkgs_file: PathBuf = Path::join(&config_directory, "pkgs.pnix");


    // Recover saved data
    let mut save_data = match std::fs::read_to_string(&pkgs_file) {
        Ok(data) => { if let Ok(recovered) = serde_json::from_str::<SavedData>(&data) { recovered }
            else { eprintln!("Couldn't read package data from {}. Your file might be corrupt", &pkgs_file.to_string_lossy()); exit(1); }
        }
        Err(_) => { SavedData { packages : vec![] } }
    };

    if !config_directory.exists() {
        match std::fs::create_dir_all(&config_directory) {
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Ok(_) => {}
            Err(e) => {
                panic!(
                    "Couldn't create a configuration directory at {:?}. Aborting...",
                    config_directory
                )
            }
        }
    }

    let cli = Cli::parse();
    match &cli.command {
        Commands::Install { packages, temp } => {

            if !require_nix(){
                eprintln!("Nix is not installed. Are you running on the PIE ?");
                exit(1);
            }

            for package in packages {
                if !install_package(package) { continue; } //Don't log packages whose installation failed
                if *temp || save_data.packages.contains(package){ continue; } //Skip installed, and don't log temp packages
                save_data.packages.push(package.clone()); //Add to packages
            }
        }
        Commands::Remove { packages, temp } => {
            if !require_nix(){
                eprintln!("Nix is not installed. Are you running on the PIE ?");
                exit(1);
            }

            for package in packages{
                if !remove_package(package) { continue; } //Don't log broken package removal
                if *temp { continue; } //Don't log temporary removal
                save_data.packages = save_data.packages.iter().filter(|&x| x != package).map(|x| x.clone()).collect();
            }
        }
        Commands::Reload {} => {
            if !require_nix(){
                eprintln!("Nix is not installed. Are you running on the PIE ?");
                exit(1);
            }

            for package in &save_data.packages {
                install_package(package);
            }
        }
        Commands::ListPackages {} => {
            for package in &save_data.packages {
                println!("{}", package);
            }
        }

        /*
        Commands::SetInstallOrder {} => {

            todo!("Make setinstallorder command");

            crossterm::execute!(stdout(), EnterAlternateScreen);
            crossterm::execute!(stdout(), Clear(ClearType::All));


            for package in save_data.packages {
                println!("\t{}", package);
            }

            if event::poll(std::time::Duration::from_millis(100)).expect("Couldn't poll events") {
                if let Event::Key(key) = event::read().unwrap(){
                    match key.code {
                        KeyCode::Up => {
                            crossterm::execute!(stdout(), MoveUp(1));
                        }
                        KeyCode::Down => {
                            crossterm::execute!(stdout(), MoveDown(1));
                        }
                        _ => {}
                    }
                }
            }

            loop {


            }

            crossterm::execute!(stdout(), LeaveAlternateScreen);
        }
        */
    }

    // Save data to ~/.pie-nix/pkgs.pnix
    if let Err(_) = std::fs::write(&pkgs_file, serde_json::to_string(&save_data).unwrap().into_bytes()) {
        eprintln!("Error writing data to file {}", pkgs_file.to_string_lossy());
    }
}
