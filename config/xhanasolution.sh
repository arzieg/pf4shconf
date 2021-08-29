#!/bin/bash
#set -x
while IFS=$'\t' read -r solutionversion sid arc sidname tag
do 
  #mandatory=${mandatory//$'\r'/}
  tag=${tag//$'\r'/}
  #echo "../target/debug/xhana add parametertemplate "
  #echo "-c $ver "
  #echo "-i \"$info\""
  #echo "-n $parameter"
  #echo "-s $scope"
  #echo "-o $iotype"
  #echo "-m $mandatory"
  #echo "-t $typ" 
  #echo "-a $arc"
  ../target/debug/xhana add solution -s $sid -n "$sidname" -d $solutionversion -a "$arc" -t "$tag"
done < xhanasolution.conf

