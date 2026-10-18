# Applet de batterie DualSense pour COSMIC

Une applet pour afficher dans le panneau COSMIC le niveau de batterie d’une
manette DualSense détectée par UPower.

## Fonctionnalités

- Affiche le pourcentage et une icône qui suit le niveau de charge.
- Affiche un état distinct quand la manette est déconnectée ou que l’état de
  la batterie ne peut pas être lu.
- Recherche le périphérique UPower à chaque vérification : aucun identifiant
  de batterie propre à une machine n’est codé en dur.
- Actualise l’affichage toutes les cinq secondes.
- Affichage non cliquable.

## Prérequis

- COSMIC Desktop et son panneau.
- Rust et Cargo.
- `upower`, disponible dans le `PATH` de l’applet.
- Les dépendances système de compilation de `libcosmic`, notamment les
  en-têtes de développement de `libxkbcommon`.
- `just` pour utiliser les commandes d’installation ci-dessous.

## Compiler et installer

Pour compiler et installer dans le compte utilisateur courant :

```sh
just install-user
```

Puis ajoute **Batterie de la DualSense** dans les réglages du panneau COSMIC.
Si l’applet n’apparaît pas dans la liste, redémarre le panneau ou ta session.

Pour désinstaller :

```sh
just uninstall-user
```

Pour compiler sans installer :

```sh
cargo build --release
```

## Tests et vérifications

```sh
cargo test
cargo clippy --all-targets -- -D warnings
```

Les tests couvrent la reconnaissance du périphérique UPower et l’analyse du
pourcentage de batterie. La vérification sur du matériel réel nécessite une
DualSense et UPower fonctionnel ; elle n’est pas remplacée par ces tests.

## Dépannage

- Vérifie que `upower -e` fonctionne dans un terminal.
- Vérifie que `upower -i <chemin-du-périphérique>` fournit un champ `percentage`
  pour la manette.
- Si l’applet affiche « Indisponible », consulte les logs utilisateur du
  processus COSMIC et vérifie que `upower` est accessible depuis son
  environnement.
- Après une mise à jour, redémarre le panneau COSMIC si l’ancienne version
  reste affichée.

## À propos du développement

Ce projet est **vibe-coded** : il a été développé de façon itérative en
décrivant le résultat souhaité à un assistant IA, qui a aidé à écrire et
modifier le code. Le code a été compilé et les tests automatisés ont été
exécutés, mais cela ne garantit pas l’absence de défauts. Les retours,
vérifications et contributions restent les bienvenus.
