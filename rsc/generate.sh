#!/bin/bash

for r in `seq 0 16 255`
do
    for g in `seq 0 16 255`
    do
        for b in `seq 0 16 255`
        do
            R=`printf '%02X' "$r"`
            G=`printf '%02X' "$g"`
            B=`printf '%02X' "$b"`
            convert -size 128x128 xc:"#${R}${G}${B}" P${R}${G}${B}.png
        done
    done
done
