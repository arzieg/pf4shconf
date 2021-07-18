#!/bin/bash
#set -x
while IFS=$'\t' read -r ver info parameter typ mandatory
do 
  ../target/debug/xhana add parameter -c $ver -i "$info" -n $parameter -t $typ -m $mandatory
done < xhanaparameter.conf

