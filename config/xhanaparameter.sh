#!/bin/bash
#set -x
while IFS=$'\t' read -r ver info parameter scope typ mandatory
do 
  ../target/debug/xhana add parameter -c $ver -i "$info" -n $parameter -s $scope -t $typ -m $mandatory
done < xhanaparameter.conf

