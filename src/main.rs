use std::{
    io,
    path::{Path, PathBuf},
    str::FromStr,
};
use std::collections::{HashMap, HashSet};
use std::process::exit;
use clap::{Parser, Subcommand};
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

        /// If specified, will register the packages in all specified distros.
        /// Negated by --temp
        #[arg(short, long)]
        distros: Vec<String>,

        /// The install priority for the provided packages
        /// Packages with higher priority will be installed first
        #[arg(short, long, required = false)]
        priority: Option<u64>
    },

    Remove {
        /// A list of packages to remove
        #[arg()]
        packages: Vec<String>,

        /// If specified, will register the packages in all specified distros.
        #[arg(short, long, required = false)]
        distros: Vec<String>
    },

    /// Reinstall packages at boot
    Reload {
        distros: Vec<String>,
    },

    /// List managed packages
    ListPackages {
        distros: Vec<String>,
    },
}

#[derive(Serialize, Deserialize)]
struct SavedData {
    /// (distro, (package, priority)[])
    packages: HashMap<String, Vec<(String, u64)>>
}

fn main() {

    const DEFAULT_PRIORITY : u64 = 0;
    const DEFAULT_DISTRO : &'static str = "default";

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
        Err(_) => { SavedData { packages : HashMap::new() } }
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
        Commands::Install { packages, temp, distros, priority } => {

            if !require_nix(){
                eprintln!("Nix is not installed. Are you running on the PIE ?");
                exit(1);
            }

            let used_priority = priority.unwrap_or(DEFAULT_PRIORITY);
            let used_distros = match distros.len() {
                0 => { vec![String::from(DEFAULT_DISTRO)] }
                _ => { distros.clone() }
            };

            let mut to_log = HashSet::new();

            for package in packages {
                if !install_package(package) { continue; } //Don't log packages whose installation failed
                if *temp { continue; } //Skip installed, and don't log temp packages

                to_log.insert(package);
            }

            for distro in used_distros {
                // Contains corresponding set, or the empty set if no set exists
                let mut set = save_data.packages.insert(distro.clone(), vec![]).unwrap_or_default();

                for package in packages {
                    set = set.into_iter().filter(|(name, priority)| name != package).collect();
                    set.push((package.clone(), used_priority))
                }

                save_data.packages.insert(distro.clone(), set);
            }
        }
        Commands::Remove { packages,  distros} => {

            let distro_vec = if distros.len() != 0 { distros.clone() } else { save_data.packages.keys().map(|x| x.clone()).collect() };
            for distro in distro_vec {
                let mut set = save_data.packages.insert(distro.clone(), vec![]).unwrap_or_default();

                // Remove packages
                set = set.into_iter().filter(|(name, _)| !packages.contains(name)).collect();

                save_data.packages.insert(distro.clone(), set);
            }
        }
        Commands::Reload { distros } => {
            if !require_nix(){
                eprintln!("Nix is not installed. Are you running on the PIE ?");
                exit(1);
            }

            let mut to_install = vec![];

            let d = if distros.len() != 0 { distros.clone() } else { save_data.packages.keys().map(|x| x.clone()).collect() };

            // Retrieve packages from distros
            for distro in d {
                if let Some(contents) = save_data.packages.get(&distro) {
                    to_install.extend(contents);
                }
            }


            // Sort before installing
            to_install.sort_by_key(|(_, p)| p );


            // Install
            for (package, _) in &to_install {
                install_package(package);
            }
        }
        Commands::ListPackages { distros } => {

            let d = if distros.len() != 0 { distros.clone() } else { save_data.packages.keys().map(|x| x.clone()).collect() };

            for distro in d {
                if let Some(contents) = save_data.packages.get(&distro) {
                    println!("===== Distro : {}", distro);
                    if let Some(contents) = save_data.packages.get(&distro) {
                        contents.iter().for_each(|(name, priority)| println!("{}, priority : {}", name, priority));
                    }
                    println!();
                }
            }
        }
    }

    // Save data to ~/.pie-nix/pkgs.pnix
    if let Err(_) = std::fs::write(&pkgs_file, serde_json::to_string(&save_data).unwrap().into_bytes()) {
        eprintln!("Error writing data to file {}", pkgs_file.to_string_lossy());
    }
}
