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

ÀFAIRE

# Outils

En plus de l'exécutable principale, quelques utilitaires sont fournis avec le package.

## convertrgb565

Cet utilitaire se charge de convertir des images aux formats habituels comme le
`PNG`/`JPEG`/... en un format binaire RGB565 lisible par la Longan Nano pour
s'afficher sur l'écran de la carte.
