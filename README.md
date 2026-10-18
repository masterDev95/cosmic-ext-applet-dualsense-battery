# Applet de batterie DualSense pour COSMIC

Cette applet affiche le pourcentage de batterie de la manette DualSense dans le
panneau COSMIC. Elle utilise UPower, comme le script `dualsense-bat`, mais
découvre le périphérique à chaque lecture au lieu de dépendre d'un identifiant
UPower propre à une installation. Si aucune manette n'est trouvée, elle affiche
« Déconnectée ». En cas d'erreur UPower, elle affiche « Indisponible ».

La batterie est actualisée toutes les cinq secondes.

## Prérequis

- COSMIC Desktop
- Rust et Cargo
- `upower`
- Les dépendances de compilation de `libcosmic` (dont `libxkbcommon`)

## Compiler et installer pour l'utilisateur courant

Depuis le dossier du projet :

```sh
just install-user
```

Redémarrer ensuite le panneau COSMIC, puis ajouter **Batterie de la DualSense**
dans les réglages du panneau. Pour désinstaller :

```sh
just uninstall-user
```

## Vérification

```sh
cargo test
```
