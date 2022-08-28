# RecetteDHistoire

Quelques outils pour décrire les formats de baladodiffusion d'histoires à
télécharger dans les boîtes à histoires pour les enfants.

Les sources, que l'on trouve dans le répertoire `recettes`, sont décrites dans
un fichier de «recette» au format yaml.

`RecetteDHistoire` va ensuite se charger de télécharger les média (s'ils se
trouvent sur le web, «ripper» le cd si c'est un livre avec CD audio) puis les
formater pour les télécharger dans la boîte à histoire.

Pour le moment deux boîtes à histoires sont visées par `RecetteDHistoire` :

- [Le raconteur](http://www.fabienm.eu/raconteur): Boîte à histoire «faite
  maison» à partir d'une [Longan Nano](http://www.fabienm.eu/wordpress/?p=1336)
  et d'un lecteur mp3
  [DFR0299](https://wiki.dfrobot.com/DFPlayer_Mini_SKU_DFR0299).
- Clef USB: Beaucoup de «minichaîne» audio sont capable de lire les mp3 de nos
  jours, la clef USB peut-être vue comme une boîte à histoire générique.

À plus long terme, il serait intéressant d'inclure les boîtes à histoires
[LUNII](https://lunii.com/fr-fr/) et [Merlin](https://www.hello-merlin.com/).

# Compilation et installation

- Cloner le dépot :

```
$ git clone https://github.com/Martoni/RecetteDHistoire.git
```

- compiler les outils avec cargo:

```
$ cd RecetteDHistoire/rdhist
$ cargo build
```

- Lancer le help du binaire principal :

```
$ ./target/debug/rdhist --help
Recette d'Histoire 0.1
Fabien Marteau <mail@fabienm.eu>
Tous les ustensiles nécessaire pour cuisiner des histoires

USAGE:
    rdhist [OPTIONS]

OPTIONS:
    -h, --help                  Print help information
    -l, --listerecettes         Liste les recettes disponibles
    -r, --recolter <RECETTE>    Récolte les ingrédients de la recette donnée en argument
    -V, --version               Print version information
```

- Un répertoire dans le home `.local/share/rdhist/` contient toutes les recettes «officielles». Le plus simple est de créer ce répertoire et d'y faire un lien vers le répertoire recettes :

```
$ mkdir -p ~/.local/share/rdhist
$ cd ~/.local/share/rdhist
$ ln -s $RDHISTPRJ/recettes
```

- On peu ensuite lister les recettes disponibles:

```
$ ./target/debug/rdhist -l
"Lili et la graine magique"
"Un voyage extraordinaire"
"Tiens bon, petite panthèse !"
```

- Puis récolter les ingrédients dans sa cagette :

```
$ ./target/debug/rdhist -r "Lili et la graine magique"
```

- Le indrédients sont téléchargés/extrait dans le répertoire `.local/share/rdhist/cagette`

```
$ ls ~/.local/share/rdhist/cagettes/
2021_04_01_Lili_et_la_graine_magique
$ ls ~/.local/share/rdhist/cagettes/2021_04_01_Lili_et_la_graine_magique/
histoire_principale.mp3  illustration_podcast.jpg
```

# Outils

En plus de l'exécutable principal `rdhist`, quelques utilitaires sont fournis avec le package.

## convertrgb565

Cet utilitaire se charge de convertir des images aux formats habituels comme le
`PNG`/`JPEG`/... en un format binaire RGB565 lisible par la Longan Nano pour
s'afficher sur l'écran de la carte.

## ciseauxmp3

Cet outil découpe et assemble des fichiers musicaux au format mp3. L'exécutable
utilise la ligne de commande pour lancer les utilitaires
[cat](http://www.linuxcertif.com/man/1/cat/) et [ffmpeg](https://ffmpeg.org/),
il est donc nécessaire de les avoir installés pour l'utiliser.
