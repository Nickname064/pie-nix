# PIE-NIX
*Un gestionnaire de paquets pour les machines du PIE (Parc Informatique Epita)*

## Motivations
En tant qu'étudiant, j'ai souvent eu besoin/envie d'installer des paquets tiers sur les machines de l'EPITA.  
Etant donné que `nix-env` est bloqué, la seule alternative, `nix profile install` nécéssite de réinstaller les paquets à chaque redémarrage.  
J'ai donc crée, pour mon usage personnel, un outil pour résoudre ce problème.  

## Fonctionnalités

- Installation de paquets NixOS
  - Gestion de priorités d'installation
  - Distributions (groupes de paquets)

## Setup

### Installer PIE-NIX

```bash
git clone https://github.com/Nickname064/pie-nix.git
cd pie-nix
cargo build --release
mv ./target/release/pie-nix ~/afs/.pie-nix
```

La commande `pie-nix` peut mainteant être trouvée dans le dossier actuel.  
Vous pouvez également supprimer le dossier récupéré via Git.

Etant donné que vous travaillez sur les ordis de l'EPITA, il est nécéssaire de stocker l'exécutable dans l'`AFS`.  
J'ai personnellement un dossier `bin` dans l'AFS, qui stocke toutes mes commandes.  
Si vous voulez faire de même, pensez à l'ajouter à votre `$PATH`.

Créer un dossier `.bin` :
```bash
  mkdir ~/afs/.bin
  ln -snf ~/afs/.bin ~/.bin
  mv ./pie-nix ~/afs/.bin
```

Ajouter le dossier à votre `$PATH`: (mettez les lignes suivantes dans votre `~/.bashrc`)
```bash
# Replace ~/afs/bin with the path to your directory for custom binaries
export PATH=~/afs/.bin:$PATH
```

## Usage

`pie-nix` permet d'installer des packages et de les configurer d'une manière qui persiste entre les boots.  
Une invocation `nix profile install [source]#[paquet]` installe le paquet, mais jusqu'a ce que vous redémarriez.  

### Installer des paquets

- Pour installer un paquet `pie-nix install [paquet]` ou `pie-nix -i [paquet]`

__Note__: Vous ne pouvez installer que des paquets qui existent sous forme de `flake`.
Par exemple, étant donné que vous pouvez installer `neovim` avec `nix profile install nixpkgs#neovim`, il est possible de l'installer avec `pie-nix`.  
Si vous souhaitez chercher des `flakes` NixOS valides, veuillez vous référer à (https://search.nixos.org/packages)

__Note(2)__: Cette commande installe un paquet localement, et en prend note de sorte à pouvoir le réinstaller automatiquement au prochain démarrage.

- Si vous voulez juste essayer un paquet, sans qu'il soit enregistré (et que vous ne souhaitez pas utiliser `nix profile install` ou `nix-shell -p`), vous pouvez utiliser `pie-nix install [paquet] --temp`
- Enfin, si vous voulez désinstaller un paquet, utilisez `pie-nix remove [paquet]` (Vous pouvez aussi l'enlever temporairement en utilisant `--temp`)

#### Priorités d'installations

Vous pouvez aussi spécifier dans quel ordre vous souhaitez que les paquets soient installés.
Pour ce faire, utilisez simplement `pie-nix install [paquet(s)] --priority [priorité]` (les paquets avec une plus haute priorité sont installés en premier)

#### Distros
Les distros sont une fonctionnalité spéciale qui vous permet de grouper des paquets.

Chaque commande qui manipule les paquets peut prendre en argument `--distros [DISTROS...]`

- `Install` n'ajoutera les paquets qu'aux distros spécifiées (au lieu de celle par défaut)
- `Remove` n'enlèvera les paquets qu'aux distros spécifiées (au lieu de celle par défaut)
- `List-Packages` ne listera que les paquets appartenant aux distros spécifiées (au lieu de toutes)
- `Reload` ne réinstallera que les paquets venant des distros spécifiées (au lieu de celle par défaut)

## Avertissements / Usages
`pie-nix` stocke ses fichiers de configuration (comme les données de paquets) dans `~/afs/.pie-nix`.  
Trafiquer ce fichier peut casser votre installation `pie-nix`, alors faites-le à vos risques et périls.   
Il est possible (et recommandé) de conserver une sauvegarde de ces données en utilisant un logiciel de gestion de version, comme Git.


