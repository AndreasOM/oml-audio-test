#!/bin/sh

ffmpeg -y -i data/coin.wav -ar 48000 -acodec pcm_s16le data/coin48000.wav
ffmpeg -y -i data/sine440hz.wav -ar 48000 -acodec pcm_s16le data/sine440hz48000.wav

omt-soundbank build --input content/test.soundbank --output data/test.omsb --use-version 3
