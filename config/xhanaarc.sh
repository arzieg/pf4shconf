#!/bin/bash
#set -x
while IFS=$'\t' read -r sid arc
do 
  ../target/debug/xhana add architecture -s $sid -a $arc
done < xhanaarc.conf

