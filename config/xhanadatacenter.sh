#!/bin/bash
#set -x
while IFS=$'\t' read -r id name
do 
  ../target/debug/xhana add datacenter -i $id -d $name
done < xhanadatacenter.conf

