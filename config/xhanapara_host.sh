#!/bin/bash
#set -x
while IFS=$'\t' read -r ver parameter hostname value
do 
  value=${value//$'\r'/}
  #echo "../target/debug/xhana add config "
  #echo "-p $ver "
  #echo "-n $parameter"
  #echo "-h $hostname"
  #echo "-v $value"
  ../target/debug/xhana add config -p $ver -n $parameter -h $hostname -v $value
done < xhanapara_host.conf

