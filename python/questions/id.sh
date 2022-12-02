#!/bin/bash
# @generator          :  template.sh
# @filename           :  id.sh
# @author             :  Copyright (C) Church.ZHONG
# @date               :  Fri Nov 11 10:00:42 AM HKT 2022
# @function           :  automatically do something what you want by shell.
# @see                :  GNU bash, version 5.0.17
# @require            :  OpenSSL 1.1.1f  31 Mar 2020/Ubuntu 20.04.2 LTS/
#set -x
SECONDS=0
EX_OK=0
EX_USAGE=64

set -o errexit
set -o nounset
set -o pipefail

# get absolute path of a path name
function abspath() {
	[[ -n "$1" ]] && (cd "$1" 2>/dev/null && pwd)
}

function err() {
	echo "# [$(date +'%Y-%m-%dT%H:%M:%S%z')]: $*" >&2
}

function log() {
	echo "$*" >&2
}

function cleanup() {
	echo "cleanup and exit"
	rc=$1
	exit "${rc}"
}

WORK_DIR=$(abspath "$(dirname "$0")")
echo "# WORK_DIR = ${WORK_DIR}"
OS_DATE_DAY=$(date +%Y-%m-%d)
echo "# OS_DATE_DAY = ${OS_DATE_DAY}"
OS_DATE_SECOND=$(date "+%Y_%m_%d_%H%M%S")
echo "# OS_DATE_SECOND = ${OS_DATE_SECOND}"
OS_DATE_NEXT_DAY=$(date -d next-day +%Y-%m-%d)
echo "# OS_DATE_NEXT_DAY = ${OS_DATE_NEXT_DAY}"
# readonly DATE=$(date)
readonly DATE="Wed Nov 16 12:31:09 PM HKT 2022"
readonly OUTPUT_DIR="${WORK_DIR}/../church"

# -------------------------------- main --------------------------------
function write_rust_by_id() {
	local id="$1"
	echo "${id}"
	# echo "${OUTPUT_DIR}"
	{
		cat <<ID_CODE
// @filename   : stackoverflow_${id}.rs
// @date       : ${DATE}
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/${id}
// @title      :

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer3 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
pub fn test() {
    _enter!();
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
ID_CODE
	} >"${OUTPUT_DIR}/src/stackoverflow_${id}.rs"
}

function write_rust_by_index() {
	local filename="$1"
	local id="$2"
	# echo "${filename}:${id}"
	# echo "${OUTPUT_DIR}"
	{
		cat <<INDEX_CODE
mod stackoverflow_${id};
INDEX_CODE
	} >>"${filename}"
}

function main() {
	mkdir -p "${OUTPUT_DIR}"
	local MAIN_RUST="${OUTPUT_DIR}/src/main.rs"
	echo -e "#[macro_use]\nmod util;" >"${MAIN_RUST}"

	find "${WORK_DIR}" -type f -name "0*.html" | sort | while read -r htmlfile; do
		echo "htmlfile = ${htmlfile}"
		local base=""
		base=$(basename "${htmlfile}")
		local index=${base%.*}
		# echo "index = ${index}"
		# cargo new stackoverflow_${index}
		local filename="${OUTPUT_DIR}/src/html_${index}.rs"
		echo "filename = ${filename}"
		/bin/true >"${filename}"
		awk '{match($5, /^data-post-id="([0-9]+)"$/, list); if(""!=list[1])print list[1]}' "${htmlfile}" | while read -r id; do
			# /bin/true
			write_rust_by_id "${id}"
			# write_rust_by_index "${filename}" "${id}"
			echo "mod stackoverflow_${id};" >>"${filename}"
		done
		local COUNT=0
		COUNT=$(awk '/mod stackoverflow_/{count++}END{print count}' "${filename}")
		if [[ 50 -ne ${COUNT} ]]; then
			echo "# COUNT = ${COUNT}, filename = ${htmlfile}"
		fi

		{
			echo "// ${index}.html"
			cat "${filename}"
			echo "mod html_${index} {"
			echo "    pub fn test() {"
			echo "        //_enter!();"
			awk 'match($0, /stackoverflow_[0-9]+/){print substr($0, RSTART, RLENGTH)}' "${filename}" | while read -r module; do
				echo "        super::${module}::test();"
			done
			echo "        //_leave!();"
			echo "    }"
			echo -e "}\n"
		} >>"${MAIN_RUST}"
		rm -f "${filename}"
		# exit 0
	done

	cp -f "${WORK_DIR}/../util.rs" "${OUTPUT_DIR}/src/util.rs"
	# TEMPORARY=$(mktemp)
	{
		echo "fn main() {"
		echo "    _enter!();"
		echo "    println!(\"Hello, world!\");"
		echo "    if cfg!(feature = \"html_001\") {"
		echo "        html_001::test();"
		for i in $(seq 2 60); do
			align=$(printf "%03d" "${i}")
			# echo ${align}
			echo "    } else if cfg!(feature = \"html_${align}\") {"
			echo "        html_${align}::test();"
		done
		echo "    }"
		echo "    _leave!();"
		echo "}"
	} >>"${MAIN_RUST}"
}

main "$@"

log "# Done"
# do some work( or time yourscript.sh)
DURATION=${SECONDS}
log "# $((DURATION / 60)) minutes and $((DURATION % 60)) seconds elapsed."
cleanup ${EX_OK}
# -------------------------------- exit --------------------------------
# curl -sS https://webinstall.dev/shfmt | bash
# shfmt -w -ln=posix /data/rust/questions/id.sh
# sudo snap install shfmt
