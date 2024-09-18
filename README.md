# PIE-NIX
*A custom Nix configuration manager, for EPITA machines*

## Motivation
As a student, I often wanted to install external nix packages, but had to install them on every machine change/reboot.
As such, I wanted to make a tool that could fix this problem and serve as a dotfiles manager for EPITA computers

## Requirements
- A Git repository hosted on a distant server (github is fine, and even recommended)

## Setup

### Building the PIE-NIX

```bash
git clone https://github.com/Nickname064/pie-nix.git
cd pie-nix
cargo build --release
mv ./target/release/pie-nix .
```

The pie-nix command can now be found in the current directory.
Since you're working on a EPITA machine, you should move it to a persistent folder, somewhere in the AFS
I personnaly have a special folder just for my executables, in /afs/bin, symlinked to be accessible from ~/.bin

```bash
  mkdir ~/afs/bin
  ln -s ~/.bin ~/afs/bin
  mv ./pie-nix ~/afs/bin
```

If you have such a directory, make sure to include it in your `PATH` environment variable.
To do so, just add the following lines to your .bashrc (which can be found at `~/.bashrc`)

```bash
# Replace ~/afs/bin with the path to your directory for custom binaries
export PATH=$PATH:~/afs/bin
```

Custom-build binaries are the only thing we are going to actually store in the afs, since all machines have the same architecture, and it would be a pain to actually recompile all of them every single time

### Setting up the PIE-NIX

`pie-nix` requires a distant Git repository to store your data. 
For now, only GitHub HTTPS authentication (with token) is supported.

To setup the distant repo, use `pie-nix --setup` like so
```bash
pie-nix --setup [GitHub url] [GitHub ID] [GitHub AUTH token]
```
example: `pie-nix setup https://Nickname064/my-cool-repo Nickname064 1234FqffsqfqNotAnActualToken`

Once this is done, you are ready to use `pie-nix` !

### Usage

`pie-nix` allows to install packages and configure them in a way that persists between boots.
A typical `nix profile install [source]#[my_cool_package]` installs the package, but only until you reboot on a different machine.
This is a pain for anyone that would like to use the computer instead of setting it up all the time.

#### Installing packages

- To install a package, use
`pie-nix install [package_name]` or `pie-nix -i [package_name]`

__Note__: You can only install packages that exist as flakes.
For example, since you can install neovim with `nix profile install nixpkgs#neovim`, installing it with `pie-nix` is fine.
If you would like to search valid nix packages, please refer to [https://search.nixos.org/packages]

__Note(2)__: This command installs a package locally, and keeps record of it so that it can be reinstalled next boot.

- If you would like to just try a package out, without pushing it to the repository, you can use
`pie-nix install [package_name] --no-backup`

- Finally, if you would like to uninstall a package, use
`pie-nix remove [package_name]`

#### Backing up files

`pie-nix` also allows you to backup your configurations in the remote repository.
TODO: ACTUALLY ALLOW BACKING UP FILES

#### Syncing changes

By default, `pie-nix` will autosync every installed/remove package.
This behavior can, as seen previously, be prevented using `--local` on every install, but it can also be changed

To do so, use `pie-nix set-sync --packages manual`
Default behavior can be restored using `pie-nix set-sync --packages auto`

For performance and readability reasons, `pie-nix` will not auto-sync files.

You can manually sync files & packages using `pie-nix`:
`pie-nix sync [--files|--packages]`
You can, of course, sync both at once: `pie-nix sync --files --packages`

## Warnings and best practices

`pie-nix` stores configuration files in `~/.pie-nix`
Tampering with this file can break your `pie-nix` install, so do so at your own risk.
