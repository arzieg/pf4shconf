#!/bin/bash
#set -x
while IFS=$'\t' read -r ver parameter solution value
do 
  #mandatory=${mandatory//$'\r'/}
  value=${value//$'\r'/}
  #echo "../target/debug/xhana add config "
  #echo "-p $ver "
  #echo "-n $parameter"
  #echo "-o $solution"
  #echo "-v $value"
  ../target/debug/xhana add config -p $ver -n $parameter -o $solution -v $value
done < xhanapara_general.conf

