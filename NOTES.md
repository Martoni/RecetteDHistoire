# Prise de notes

## Extractions d'un cd audio

Avec [cdparanoia](https://linuxconfig.org/how-to-rip-an-audio-cd-from-the-command-line-using-cdparanoia) :

```
$ cdparanoia -XB
```

## Conversion wav -> mp3

Avec [ffmpeg](https://lonewolfonline.net/convert-wav-mp3-linux/) :

```
$ ffmpeg -i track01.cdda.wav -acodec mp3 track01.cdda.mp3
```
