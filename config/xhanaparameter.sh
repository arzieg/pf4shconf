#!/bin/bash
#set -x
while IFS=$'\t' read -r ver info parameter scope iotype typ mandatory arc
do 
  #mandatory=${mandatory//$'\r'/}
  arc=${arc//$'\r'/}
  #echo "../target/debug/xhana add parametertemplate "
  #echo "-c $ver "
  #echo "-i \"$info\""
  #echo "-n $parameter"
  #echo "-s $scope"
  #echo "-o $iotype"
  #echo "-m $mandatory"
  #echo "-t $typ" 
  #echo "-a $arc"
  ../target/debug/xhana add parametertemplate -v $ver -i "$info" -n $parameter -s $scope -o $iotype -t $typ -m $mandatory -a $arc
done < xhanaparameter.conf

