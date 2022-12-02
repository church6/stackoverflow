#!/bin/bash
# @generator          :  template.sh
# @filename           :  pdca.sh
# @author             :  Copyright (C) Church.ZHONG
# @date               :  Wed Nov 16 03:43:49 PM HKT 2022
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

readonly target="006"
# -------------------------------- main --------------------------------
function main() {
	awk -v align="${target}" 'BEGIN{range=0}{
	if($1 == "//" && $2 == align ".html") {range=1}
	if($1 == "mod" && $2 == "html_" align) {range=0}
	if (range) {
		if (match($0, /stackoverflow_[0-9_]{*}/)) {print "https://stackoverflow.com/questions/" substr($0, RSTART + 14)}
	}
	}' "${WORK_DIR}/src/main.rs" | while read -r line; do
		echo "${line}"
		#open "${line}"
		#sleep 1.5
	done
	# >"${WORK_DIR}/urls.txt"

	sed -i -E "s/(default = [\"html_[0-9]+\"])/default = [\"html_${target}\"]/" "${WORK_DIR}/Cargo.toml"
}

main "$@"

log "# Done"
# do some work( or time yourscript.sh)
DURATION=${SECONDS}
log "# $((DURATION / 60)) minutes and $((DURATION % 60)) seconds elapsed."
cleanup ${EX_OK}
# -------------------------------- exit --------------------------------
# curl -sS https://webinstall.dev/shfmt | bash
# shfmt -w -ln=posix /data/rust/stackoverflow/church/pdca.sh
# sudo snap install shfmt
