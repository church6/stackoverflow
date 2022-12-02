#!/bin/bash

echo "[features]"
echo "default = [\"html_001\"]"

for i in $(seq 1 60); do
	align=$(printf "%03d" "${i}")
	# echo ${align}
	echo "html_${align} = []"
done
