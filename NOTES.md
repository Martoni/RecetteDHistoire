# Prise de notes

## Extractions d'un cd audio

Avec [cdparanoia](https://linuxconfig.org/how-to-rip-an-audio-cd-from-the-command-line-using-cdparanoia) :

```
$ cdparanoia -XB
```

## Identification d'un cd

- Brute

```
$ cd-discid
30027f05 5 150 2422 15072 29219 44022 641
```

- Table des matières

```
$ cdparanoia -Q
cdparanoia III release 10.2 (September 11, 2008)

 

Table of contents (audio tracks only):
track        length               begin        copy pre ch
===========================================================
  1.     2272 [00:30.22]        0 [00:00.00]    no   no  2
  2.    12650 [02:48.50]     2272 [00:30.22]    no   no  2
  3.    14147 [03:08.47]    14922 [03:18.72]    no   no  2
  4.    14803 [03:17.28]    29069 [06:27.44]    no   no  2
  5.     4069 [00:54.19]    43872 [09:44.72]    no   no  2
TOTAL   47941 [10:39.16]    (audio only)
```

## ejection

```
eject
```

## Conversion wav -> mp3

Avec [ffmpeg](https://lonewolfonline.net/convert-wav-mp3-linux/) :

```
$ ffmpeg -i track01.cdda.wav -acodec mp3 track01.cdda.mp3
```
