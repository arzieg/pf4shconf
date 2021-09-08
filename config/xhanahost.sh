#!/bin/bash
#set -x
while IFS=$'\t' read -r sid hostname dcid
do 
  dcid=${dcid//$'\r'/}
  #echo "../target/debug/xhana add host "
  #echo "-s $sid "
  #echo "-h $hostname"
  #echo "-i $dcid"
  ../target/debug/xhana add host -s $sid -h $hostname -i $dcid
done < xhanahost.conf

