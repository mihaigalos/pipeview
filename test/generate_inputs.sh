#!/bin/bash

MAX=1000

while :; do
  # RANDOM=$(date +%s%N | cut -b10-19)
  # echo $(( ( $RANDOM % 10 ) + 1 ))
  v=$[100 + (RANDOM % 100)]$[1000 + (RANDOM % 1000)]
  v=$[RANDOM % $MAX].${v:1:2}${v:4:3}
  echo $v
  sleep 0.3
done
