#!/bin/bash
#set -x
while IFS=$'\t' read -r ver info parameter scope iotype typ mandatory
do 
  echo ../target/debug/xhana add parameter -c $ver -i "$info" -n $parameter -s $scope -o $iotype -m $mandatory -t $typ 
  ../target/debug/xhana add parameter -c $ver -i "$info" -n $parameter -s $scope -o $iotype -t $typ -m $mandatory
done < xhanaparameter.conf

