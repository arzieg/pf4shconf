#!/bin/bash
#set -x
while IFS=$'\t' read -r arc
do 
  ../target/debug/xhana add architecture -a $arc
done < xhanaarc.conf

