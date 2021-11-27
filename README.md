# RecetteDHistoire
Some tools to describe podcast and format it for children story boxes. Mainly
written for the [Raconteur](http://fabienm.eu/raconteur/).

Sources (audio file, pictures, ...) are described in yaml format (extension
`.rdhist`) in directory recettes.

`RecetteDHistoire` can then generate binaries images for some story box like
the [Raconteur](http://fabienm.eu/raconteur/) or USB-key (for the moment).

The objective is to support as many story boxes as possible.

# Compile and install

TODO

# Binaries

## `convertrgb565`

This tools is used to convert images from usual format like `PNG`, `JPEG`, ...
to a binary raw RGB565 suitable for [Longan
Nano](https://www.seeedstudio.com/Sipeed-Longan-Nano-RISC-V-GD32VF103CBT6-Development-Board-p-4205.html)
oled LCD.

