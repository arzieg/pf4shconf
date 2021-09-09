#!/bin/bash
#set -x
while IFS=$'\t' read -r sid hostname dcid arc
do 
  arc=${arc//$'\r'/}
  echo "../target/debug/xhana add host "
  echo "-s $sid "
  echo "-h $hostname"
  echo "-i $dcid"
  echo "-a $arc"
  ../target/debug/xhana add host -s $sid -h $hostname -i $dcid -a $arc
done < xhanahost.conf

