#!/bin/bash
#set -x
while IFS=$'\t' read -r ver info parameter scope iotype typ mandatory
do 
  mandatory=${mandatory//$'\r'/}
  echo "../target/debug/xhana add parameter "
  echo "-c $ver "
  echo "-i \"$info\""
  echo "-n $parameter"
  echo "-s $scope"
  echo "-o $iotype"
  echo "-m $mandatory"
  echo "-t $typ" 
  ../target/debug/xhana add parameter -c $ver -i "$info" -n $parameter -s $scope -o $iotype -t $typ -m $mandatory 
done < xhanaparameter.conf

