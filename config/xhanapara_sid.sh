#!/bin/bash
#set -x
while IFS=$'\t' read -r ver parameter sid value
do 
  value=${value//$'\r'/}
  #echo "../target/debug/xhana add config "
  #echo "-p $ver "
  #echo "-n $parameter"
  #echo "-s $sid"
  #echo "-v $value"
  ../target/debug/xhana add config -p $ver -n $parameter -s $sid -v $value
done < xhanapara_sid.conf

