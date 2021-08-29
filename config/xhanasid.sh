#!/bin/bash
#set -x
while IFS=$'\t' read -r sid name
do 
  ../target/debug/xhana add architecture -a $arc
done < xhanasid.conf

