# PIE-NIX
*A custom Nix configuration manager, for EPITA machines*

## Motivation
As a student, I often wanted to install external nix packages, but had to install them on every machine change/reboot.
As such, I made a tool for personal use that could handle this problem.

## Features
- Install packages (and keep them between reboots)
  - Package install priority ( just a number )
  - Distros (package groups)

## Setup

### Building the PIE-NIX

```bash
git clone https://github.com/Nickname064/pie-nix.git
cd pie-nix
cargo build --release
mv ./target/release/pie-nix ~/afs/.pie-nix
```

The pie-nix command can now be found in the current directory.
Since you're working on a EPITA machine, you should move it to a persistent folder, somewhere in the AFS
I personnaly have a special folder just for my executables, in /afs/bin, symlinked to be accessible from ~/.bin

```bash
  mkdir ~/afs/.bin
  ln -snf ~/afs/.bin ~/.bin
  mv ./pie-nix ~/afs/.bin
```

If you have such a directory, make sure to include it in your `PATH` environment variable.
To do so, just add the following lines to your .bashrc (which can be found at `~/.bashrc`)

```bash
# Replace ~/afs/bin with the path to your directory for custom binaries
export PATH=~/afs/.bin:$PATH
```

## Usage

`pie-nix` allows to install packages and configure them in a way that persists between boots.
A typical `nix profile install [source]#[my_cool_package]` installs the package, but only until you reboot on a different machine.
This is a pain for anyone that would like to use the computer instead of setting it up all the time.

### Installing packages

- To install a package, use
`pie-nix install [package_name]` or `pie-nix -i [package_name]`

__Note__: You can only install packages that exist as flakes.
For example, since you can install neovim with `nix profile install nixpkgs#neovim`, installing it with `pie-nix` is fine.
If you would like to search valid nix packages, please refer to [https://search.nixos.org/packages]

__Note(2)__: This command installs a package locally, and keeps record of it so that it can be reinstalled next boot.

- If you would like to just try a package out, without pushing it to the repository, you can use
`pie-nix install [package_name] --temp`

- Finally, if you would like to uninstall a package, use
`pie-nix remove [package_name]`
(You can also remove it temporarily by using `--temp`)

#### Install priorities
You can also specify in which order you would like the packages to be installed.
To do so, just specify a priority when installing a package (packages with higher priority are installed first)
ex: `pie-nix install [package] --priority 10`

This can be very useful on EPITA machines, as a command is only available once it's installed (DUH).
This means that if you need to install a very heavy package, and a package that you might need to use from the get go, 
priorities allow you to install the second package first, and the other one later.

#### Distros
Distros are a special feature, which allows you to create package groups.

Every command that manipulates packages can also take as an argument `--distros [DISTROS...]`.
- Install will only add the packages to the specified distros (instead of the default one)
- Remove will only remove the packages from the specified distros
- List-Packages will only list the packages from the specified distros (unless no distro is specified, in which case it will list all distros)
- Reload will only reload the packages from the specified distros (instead of the default ones)

This feature can be very useful if you need to install a minimal environment quickly, and can add other packages later.

## Warnings and best practices
`pie-nix` stores configuration files (such as package data) in `~/afs/.pie-nix`.
Tampering with this file can break your `pie-nix` install, so do so at your own risk.
It is possible (and generally advised) to backup the contents of this file using version control software, such as Git.