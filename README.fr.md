<h1 align="center">Quiet.md</h1>

<p align="center">
  <b>Des notes rapides et discrètes, qui vivent sur votre ordinateur.</b><br>
  Une app de notes native écrite en Rust + GPUI : de vrais dossiers, du Markdown qui se met en forme tout seul et un tableau blanc dans vos notes. Gratuite et open source.
</p>

<p align="center">
  <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.es.md">Español</a> · Français
</p>

<p align="center">
  <a href="https://github.com/s1dashu/quiet-md/releases/latest"><img src="https://img.shields.io/github/v/release/s1dashu/quiet-md?label=release&color=2f6feb" alt="Dernière version"></a>
  <a href="https://github.com/s1dashu/quiet-md/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/s1dashu/quiet-md/ci.yml?branch=main&event=schedule&label=CI" alt="État de la CI"></a>
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey" alt="macOS, Windows et Linux">
  <img src="https://img.shields.io/badge/built%20with-Rust%20%2B%20GPUI-dea584?logo=rust" alt="Faite avec Rust et GPUI">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/s1dashu/quiet-md?color=green" alt="Licence MIT"></a>
  <img src="https://img.shields.io/badge/status-beta-orange" alt="Beta">
</p>

<p align="center">
  <a href="https://download.quiet.md/latest/quiet-md-macos-arm64.dmg"><img src="docs/images/download-macos.svg" height="56" alt="Télécharger pour macOS"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe"><img src="docs/images/download-windows.svg" height="56" alt="Télécharger pour Windows"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-x86_64.AppImage"><img src="docs/images/download-linux.svg" height="56" alt="Télécharger pour Linux"></a>
</p>

<p align="center">
  <a href="https://quiet.md/?lang=fr"><b>Site web</b></a> ·
  <a href="#téléchargement"><b>Téléchargement</b></a> ·
  <a href="#fonctionnalités">Fonctionnalités</a> ·
  <a href="#performances">Performances</a> ·
  <a href="CONTRIBUTING.md">Contribuer</a>
</p>

<p align="center">
  <img src="docs/images/readme-home.webp" width="100%" alt="Quiet.md sur une prairie en pixel art : la barre latérale liste les dossiers, et la page du dossier principal montre chaque dossier comme une pile de cartes de notes illustrées.">
</p>

> [!NOTE]
> Quiet.md est en **bêta** et évolue vite. Si vous rencontrez un bug ou avez une idée, [ouvrez une issue](https://github.com/s1dashu/quiet-md/issues), ou mieux, envoyez une pull request.

## Pourquoi Quiet.md

**📁 Vos notes sont de simples fichiers.** Pas de compte, pas de cloud, et rien ne part sur le réseau par défaut (hors vérification facultative des mises à jour, comptes d'utilisation anonymes envoyés une fois par jour par l'app et l'outil en ligne de commande `quiet` — seulement des comptes, comme le nombre d'ouvertures de l'app ; à désactiver dans Réglages › Général › Statistiques d'utilisation anonymes — et polices que vous déclenchez : la police manuscrite pour le chinois, le japonais et le coréen la première fois qu'un tableau blanc en a besoin, et une police de document téléchargeable quand vous la choisissez). Chaque dossier de la barre latérale est un vrai dossier sur votre disque, et chaque note est un fichier nommé d'après son titre : `.md` pour le Markdown, `.xml` pour les documents enrichis. Synchronisez-les avec iCloud, Git ou l'outil de votre choix. Si vous arrêtez un jour d'utiliser Quiet.md, vos fichiers restent exactement où ils sont.

**⚡ Native, et rapide.** Toute l'interface est dessinée sur le GPU par Rust et [GPUI](https://www.gpui.rs/). Ni Electron, ni WebView. Dans une note d'un million de caractères, la frappe prend environ 5 ms par image, quand un écran à 120 Hz en accorde 8,3.

**✍️ Écrire et dessiner au même endroit.** Tapez `#`, `-`, `[]` ou `>` et le Markdown se met en forme tout seul. Quand les mots ne suffisent plus, placez un tableau blanc directement dans la note. Les notes enrichies sont enregistrées au format DocxXML de Feishu (Lark) : un collage depuis Feishu ou une page web garde sa structure.

## Fonctionnalités

### Écrire

<p align="center">
  <img src="docs/images/readme-write.webp" width="100%" alt="Une longue note ouverte dans Quiet.md, avec des titres, des formules en ligne et un tableau de concentrations ioniques.">
</p>

Titres, listes, tâches, citations, encadrés, colonnes, tableaux avec cellules fusionnées, code avec coloration syntaxique, formules LaTeX, images, fichiers et couvertures. Les raccourcis Markdown fonctionnent pendant la frappe, et le menu `/` propose tout le reste. Mentionnez une autre note pour y faire un lien ; elle affichera ses rétroliens. Les bases de données rangent des fiches avec des champs de choix, date, nombre, formule, recherche et lien, dans des vues que vous filtrez, triez et groupez ; chaque cellule d'une colonne Notes est une vraie note. Le menu … propose Mode concentration, Mode machine à écrire et Focus sur le paragraphe, et Présentation › Zoom avant agrandit le texte seul.

### Dessiner

<p align="center">
  <img src="docs/images/readme-board.webp" width="100%" alt="Un tableau dans Quiet.md : images et cartes disposées avec des étiquettes manuscrites, des boîtes et des flèches qui les relient.">
</p>

Les tableaux sont un tableau blanc porté d'Excalidraw : formes, flèches qui restent attachées, dessin à main levée, texte, notes adhésives, cadres et images. Placez-en un dans une note, ou gardez-le comme fichier `.excalidraw` qu'excalidraw.com sait ouvrir. Zoomez en pinçant le trackpad ; un tableau de 10 000 formes reste fluide.

### Ranger dans de vrais dossiers

La barre latérale regroupe vos dossiers de premier niveau ; ouvrez-en un, et sa page montre les sous-dossiers en piles de leurs pages, et la vue en liste se trie par n'importe quelle colonne. Ajoutez n'importe quel dossier de votre ordinateur avec **Ajouter un dossier** : il s'utilise sur place. Maintenez une note au-dessus d'un dossier et il s'ouvre tout seul ; les dossiers s'imbriquent, pages et dossiers passent d'un dossier de premier niveau à un autre, et chaque déplacement s'annule. Épinglez des notes en haut de la barre latérale, cherchez dans tous les titres et toutes les lignes depuis l'icône de recherche ou avec `⌘ J`, et faites des rechercher-remplacer dans une note, expressions régulières comprises. Les notes supprimées attendent dans la corbeille le nombre de jours que vous choisissez. Un fichier Markdown ouvert seul se modifie dans sa propre fenêtre, sur place.

### Apporter vos notes

Importer depuis une autre app… (dans le menu Fichier) cherche sur l'ordinateur vos bibliothèques Obsidian, Logseq, iA Writer et Typora et vos espaces de travail SiYuan, et les copie sans toucher aux originaux. Les `[[liens wiki]]` d'Obsidian continuent de fonctionner : cliquez sur l'un d'eux pour ouvrir la note visée. Ensuite, glissez des fichiers et dossiers Markdown sur la page d'un dossier ou directement dans un dossier de la barre latérale. Les notes repartent en Markdown ou en PDF, ou directement vers l'imprimante.

### Et aussi

- **Historique des versions** : chaque note garde des instantanés à l'enregistrement (20 par défaut, jusqu'à 30). Ouvrez-en un pour voir sur place ce qui a changé, restaurez-le en un clic ou donnez-lui un nom.
- **Notes rapides** : sur macOS, un raccourci global (`⌥ Espace` par défaut) ouvre une petite fenêtre de note par-dessus n'importe quelle app.
- **Votre style** : quatre styles intégrés — Craft, Feishu, Notion et Obsidian —, chacun avec une variante sombre, ou votre propre CSS ([docs/THEMES.md](docs/THEMES.md)). Choisissez la police de l'interface et celle des documents.
- **Vos touches** : appuyez sur `⌘ /` pour voir tous les raccourcis et réaffecter ou effacer n'importe lequel. Sous Windows et Linux, Ctrl remplace ⌘ et Alt remplace ⌥.
- **Ligne de commande et agents** : l'outil `quiet`, fourni avec l'app, permet à des agents d'IA comme Claude Code ou Codex de lire et modifier votre bibliothèque avec le même moteur que l'app. Réglages › Connecteur d'agent donne une ligne à coller à votre agent, qui s'installe tout seul ; sur macOS, l'outil s'installe en un clic. Détails dans le [README en anglais](README.md#work-with-agents).
- **Votre langue** : anglais, chinois simplifié, chinois traditionnel, japonais, coréen, espagnol, français, arabe et turc. L'arabe lie ses lettres et s'écrit de droite à gauche.
- **Ménage** : Réglages › Bibliothèque repère les images et fichiers qu'aucune note n'utilise plus et les retire une fois que vous avez confirmé.
- **Mises à jour** : les nouvelles versions s'installent depuis l'app après vérification de leur signature. Les vérifications automatiques peuvent être désactivées.

## Feuille de route

- [x] Documents riches : titres, listes, encadrés, tableaux, colonnes, code, formules, images et fichiers
- [x] Fichiers Markdown, ouverts et modifiés sur place
- [x] Tableaux blancs : schémas et croquis à main levée, dans les notes ou à part
- [x] Ouvrir n'importe quel dossier sur place, côte à côte dans la barre latérale
- [x] Import depuis Obsidian, Logseq, SiYuan, Typora, iA Writer et tout dossier Markdown
- [x] Modes d'écriture : concentration, machine à écrire et focus sur le paragraphe
- [x] Thèmes, mode sombre et neuf langues
- [x] Une CLI `quiet` et une skill pour les agents d'IA
- [ ] Bases de données (en cours)
- [ ] Diagrammes Mermaid (en cours)
- [ ] Mobile : iPhone, iPad et Android (en cours)
- [ ] Synchronisation entre appareils (en cours)
- [ ] Propriétés de document
- [ ] Capture de pages web

## Téléchargement

<p align="center">
  <a href="https://download.quiet.md/latest/quiet-md-macos-arm64.dmg"><img src="docs/images/download-macos.svg" height="56" alt="Télécharger pour macOS"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe"><img src="docs/images/download-windows.svg" height="56" alt="Télécharger pour Windows"></a>
  &nbsp;
  <a href="https://download.quiet.md/latest/quiet-md-x86_64.AppImage"><img src="docs/images/download-linux.svg" height="56" alt="Télécharger pour Linux"></a>
</p>

| Plateforme | Paquet | Prérequis |
|---|---|---|
| **macOS** (Apple Silicon) | [quiet-md-macos-arm64.dmg](https://download.quiet.md/latest/quiet-md-macos-arm64.dmg) | macOS 13 ou ultérieur |
| **Windows** (x64) | [quiet-md-windows-x64-setup.exe](https://download.quiet.md/latest/quiet-md-windows-x64-setup.exe) | Windows 10 version 2004 ou ultérieure |
| **Linux** (x86_64, expérimental) | [quiet-md-x86_64.AppImage](https://download.quiet.md/latest/quiet-md-x86_64.AppImage) | glibc 2.35 ou ultérieure (Ubuntu 22.04 et plus récent) |
| **macOS** (Intel) | [quiet-md-macos-x86_64.dmg](https://download.quiet.md/latest/quiet-md-macos-x86_64.dmg) | macOS 13 ou ultérieur |

- **macOS** : ouvrez le `.dmg`, glissez `quiet.md.app` sur le raccourci Applications à côté, puis ouvrez-la depuis Applications. L'app et l'image disque sont signées et notariées par Apple. Le `.zip` de la page Releases contient la même app ; c'est lui que télécharge la mise à jour intégrée.
- **Windows** : lancez l'installateur. Par défaut, l'installation est limitée à l'utilisateur courant, et la désinstallation ne supprime jamais vos notes. Si SmartScreen apparaît au premier lancement, cliquez sur **Plus d'infos → Exécuter quand même**.
- **Linux** (expérimental) : `chmod +x quiet-md-*.AppImage` puis lancez-le. Sans FUSE, ajoutez `--appimage-extract-and-run`. Les AppImage 0.1.10 et plus récentes prennent en charge la mise à jour depuis l'app. Les versions Linux plus anciennes nécessitent une installation manuelle unique. Les tests interactifs ont jusqu'ici porté sur Omarchy sous Wayland (voir les [notes Linux](CONTRIBUTING.md#linux-experimental)).

Vous pouvez aussi télécharger depuis [Quiet.md](https://quiet.md/?lang=fr). Chaque version et ses sommes de contrôle SHA-256 sont sur [GitHub Releases](https://github.com/s1dashu/quiet-md/releases).

Au premier lancement, Quiet.md ouvre directement un dossier **Quiet Notes** dans vos Documents (ou le dossier `Quiet.md` qu'une version précédente y a créé), avec le guide **Getting started** : une visite de dix minutes des principales fonctionnalités. Les notes existantes ne sont jamais écrasées. Ajoutez d'autres dossiers ou importez les notes d'autres apps quand vous voulez, depuis le menu Fichier.

## Performances

Temps de traitement par image sur 100 opérations consécutives dans un document mixte d'**un million de caractères** et 549 chapitres :

| Opération | P50 | P95 |
|---|---:|---:|
| ⌨️ Saisie | **5,06 ms** | 5,78 ms |
| ⌫ Retour arrière | **4,99 ms** | 5,55 ms |
| 🖱️ Défilement | **2,08 ms** | 4,14 ms |

Un écran 120 Hz laisse 8,3 ms par image. Même à P95 sur un document d'un million de caractères, il reste de la marge.

| | |
|---|---|
| Téléchargement macOS | ~18 Mo |
| Installateur Windows | ~14 Mo |
| Exécution | Pas de WebView, pas d'Electron, pas de services en arrière-plan |

<sub>Apple M5 Pro / 48 Go, macOS 26.5, build release. Méthodologie et scripts de reproduction : [docs/PERFORMANCE.md](docs/PERFORMANCE.md).</sub>

## Contributeurs

Merci à toutes les personnes qui ont contribué à construire Quiet.md. Issues et pull requests sont bienvenues : compilation, tests et conventions de code sont dans [CONTRIBUTING.md](CONTRIBUTING.md).

[![Contributeurs](https://raw.githubusercontent.com/s1dashu/quiet-md/contributors-assets/docs/images/contributors.svg?v=1)](https://github.com/s1dashu/quiet-md/graphs/contributors)

## Communauté

Si vous utilisez WeChat, scannez le code ci-dessous pour rejoindre notre groupe d’utilisateurs. Un serveur Discord arrive bientôt.

<img src="docs/images/wechat-group.webp" alt="Groupe d’utilisateurs WeChat" width="200">

## Dernières mises à jour

- **0.2.0** (28 nov) : une interface plus simple bâtie sur un seul système de design, les bases de données pour tous, trois modes d'écriture et une ligne de commande que votre agent IA peut utiliser.
  - **Les dossiers sont le niveau supérieur** : il n'y a plus de page d'accueil séparée. La barre latérale regroupe les dossiers de premier niveau sous des en-têtes repliables, et le menu … de chaque en-tête propose Gérer, Afficher dans le Finder et Retirer de la barre latérale. Ajouter un dossier utilise le dossier là où il se trouve. « Mes documents » et « Fichiers de cet ordinateur » disparaissent : un fichier Markdown ouvert seul a sa propre fenêtre, et les dossiers ouverts auparavant deviennent des dossiers de premier niveau. Glissez une page ou un dossier dans un autre dossier de premier niveau, ou utilisez Déplacer vers ; ses images, liens et historique des versions le suivent, et l'opération peut être annulée. Les Favoris rejoignent les Épinglés.
  - **Un seul système de design** : menus, boutons, boîtes de dialogue, interrupteurs, infobulles et barres de défilement forment un seul jeu de composants inspiré de shadcn/ui, avec des coins continus (squircle) sur macOS, un fin anneau et une ombre douce autour des popovers, une seule échelle d'ombres et des icônes Nucleo partout. Les menus trop longs défilent, et se diriger vers un sous-menu n'allume plus les lignes traversées. L'en-tête affiche + · Rechercher · … sur chaque page (+ · Afficher · Rechercher · … sur les dossiers), et le fil d'Ariane s'aligne sur les boutons de fenêtre d'après la hauteur des capitales.
  - **Bases de données** : créez-en une depuis le menu Nouveau, le menu `/` ou un fichier CSV. Les champs comprennent texte, sélection unique et multiple, date, nombre, formule, recherche et lien ; les vues filtrent, trient, regroupent et colorent les lignes selon des règles, avec une hauteur de ligne Ajuster au contenu. Chaque cellule d'une colonne Note est une vraie note de votre bibliothèque.
  - **Modes d'écriture**, dans le menu … du document : Mode concentration (⌘.) laisse le document seul dans sa fenêtre ; Mode machine à écrire (⌘⇧T) garde au milieu la ligne en cours ; Focus sur le paragraphe (⌘⇧F) garde le paragraphe courant net et estompe le reste. Afficher › Zoom avant / Zoom arrière (⌘+ / ⌘−) agrandit seulement le texte de la note, titres, tableaux, code et équations compris.
  - **Ligne de commande et agents** : le nouvel outil `quiet` liste, recherche, lit et modifie notes et bases de données depuis un terminal, ajoute et retire des dossiers de la barre latérale, et ouvre un document sur un bloc donné. Réglages › Connecteur d’agent configure Claude Code, Codex et d'autres agents en une commande ; `quiet` est livré avec l'app et rejoint votre PATH (Installer la commande sur macOS, l'installateur sous Windows).
  - **Un mode sombre plus lisible** : liens, couleurs de texte, surlignages, dates de rappel, comptes à rebours et libellés de boutons atteignent un contraste lisible ; la barre latérale est plus claire, et ⌃⌘D bascule entre sombre et clair.
  - **Aussi** : les sous-pages reviennent ; coller depuis Word, un navigateur ou Feishu conserve la mise en forme sous Windows et Linux ; import des espaces de travail SiYuan, avec SiYuan, Obsidian, Typora et les dossiers Markdown réunis sous Importer depuis une autre app… ; la Corbeille se vide d'elle-même après 1 à 30 jours ; tout raccourci peut être effacé ; Ouvrir dans une nouvelle fenêtre reprend la taille et la barre latérale de la fenêtre principale ; renommer ou déplacer un dossier s'affiche aussitôt, pendant que les fichiers suivent en arrière-plan ; le premier lancement ouvre directement Documents › Quiet Notes et les guides regroupés, avec des fichiers d'exemple Markdown, tableau blanc et base de données ; un comptage anonyme de l'usage quotidien, désactivable dans Réglages › Général.
  - **Format de bibliothèque** : ouvrir une bibliothèque la fait passer au FORMAT 6, après quoi la 0.1.14 et les versions antérieures ne peuvent plus l'ouvrir. Les bibliothèques dans iCloud et autres dossiers synchronisés ne sont pas mises à niveau pour l'instant.
- **0.1.14** (22 nov) : rechercher-remplacer avec des expressions régulières ; les `[[liens wiki]]` d'Obsidian ouvrent leur note ; interfaces en turc et en arabe de droite à gauche ; choix des polices de l'interface et des documents ; nettoyage des images et fichiers inutilisés ; recherche et suppression groupée plus rapides dans les grandes bibliothèques.
- **0.1.13** (17 nov) : tableaux blancs portés depuis Excalidraw, dans une note ou en fichier `.excalidraw` ; historique des versions pour chaque note ; interfaces en français et en arabe ; Copier en Markdown ; les images collées depuis le web sont enregistrées dans la bibliothèque.
- **0.1.12** (14 nov) : glissez des fichiers depuis le Finder directement dans les dossiers de la barre latérale ; cartes de l'accueil aux coins continus ; Ouvrir dans le Finder ; une largeur minimale de fenêtre qui garde de la place pour la note.
- **0.1.11** (13 nov) : la bibliothèque devient de vrais dossiers avec des fichiers nommés d'après leur titre, migrée automatiquement avec une sauvegarde ; un nouveau premier lancement avec import depuis Obsidian ; interfaces en japonais, espagnol et coréen ; équations en bloc dans le Markdown ; lancement plus rapide et moins de mémoire.
- **0.1.10** (10 nov) : choix d'un type de document dans le menu Nouveau ; les tableaux larges défilent horizontalement ; le Markdown conserve l'alignement des colonnes.
- **0.1.9** (8 nov) : corrections pour Cmd+Q sur macOS, les boutons de fenêtre sous Windows et les sélections masquées par les surlignages.
- **0.1.8** (9 nov) : édition sur place des fichiers Markdown locaux ; dossiers imbriqués avec glisser-déposer et annulation ; moins de mémoire pour les images ; une AppImage Linux expérimentale.
- **0.1.7** (6 nov) : enregistrement multi-fenêtres et récupération de conflits plus sûrs ; les exports Markdown conservent les pièces jointes en ligne.
- **0.1.6** (6 nov) : frappe et changement de fenêtre plus fluides, tableaux plus larges, ⌥ Space pour la note rapide sur macOS ; collage d'images, export PDF et impression sous Windows.
- **0.1.5** (5 nov) : l'app devient Quiet.md ; gestion de plusieurs documents à la fois ; la Corbeille se vide en une étape.
- **0.1.4** (5 nov) : mises à jour depuis l'app, vérifiées par signature ; mode sombre pour chaque thème.
- **0.1.3** (4 nov) : texte de la barre latérale lisible quand macOS est en mode sombre.
- **0.1.2** (4 nov) : les menus et raccourcis standard de macOS ; l'icône du Dock rouvre une fenêtre.
- **0.1.1** (3 nov) : première version Windows x64.

Les notes complètes de chaque version sont sur [Releases](https://github.com/s1dashu/quiet-md/releases).

## Licence

Le code est sous licence [MIT](LICENSE). Le code, les polices et les ressources tiers sont listés dans [THIRD_PARTY.md](THIRD_PARTY.md).

Les éléments suivants ne sont pas couverts par la licence MIT et doivent être remplacés dans les forks :

- Les icônes Nucleo de `assets/icons/` (82) et de `assets/editor-icons/` (24) sont sous une licence Nucleo commerciale. Ne les extrayez pas et ne les redistribuez pas seules.
- Le nom « Quiet.md » et l'icône de l'app.

## Star History

<a href="https://www.star-history.com/#s1dashu/quiet-md&type=date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=s1dashu/quiet-md&type=date&theme=dark" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=s1dashu/quiet-md&type=date" />
  </picture>
</a>
