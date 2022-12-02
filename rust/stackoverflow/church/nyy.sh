#!/bin/bash
id="$*";
echo "${id}"
readonly  input="/data/ro/hello/src/main.rs"
readonly output="/data/stackoverflow_xxx.rs"
{
awk -v id=${id} 'BEGIN{t=0}{ if($0 ~ "mod q" id " {"){t=1}if(0<t&&t<100){t++}else{t=0}if(0<t&&t<100)print $0}' "${input}"
} > "${output}"

