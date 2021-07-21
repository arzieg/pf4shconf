#!/bin/bash
#set -x
while IFS=$'\t' read -r sid configversion tag
do 
  echo ../target/debug/xhana add version -s $sid -c $configversion -t "$tag"
  if [ -z "$tag" ]
  then
    ../target/debug/xhana add version -s $sid -c $configversion 
  else
    ../target/debug/xhana add version -s $sid -c $configversion -t "$tag"
  fi 
done < xhanaversion.conf

