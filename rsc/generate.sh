#!/bin/bash

readonly TONE=4
readonly TONE_STEP=`expr 256 / $TONE`

for r in `seq 0 ${TONE_STEP} 255`
do
    for g in `seq 0 ${TONE_STEP} 255`
    do
        for b in `seq 0 ${TONE_STEP} 255`
        do
            R=`printf '%02x' "$r"`
            G=`printf '%02x' "$g"`
            B=`printf '%02x' "$b"`
            convert -size 128x128 xc:"#${R}${G}${B}" p${R}${G}${B}.png
        done
    done
done
