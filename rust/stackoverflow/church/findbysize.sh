#!/bin/bash
# @generator          :  template.sh
# @filename           :  findbysize.sh
# @author             :  Copyright (C) Church.ZHONG
# @date               :  Mon Nov 21 12:42:09 PM HKT 2022
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

readonly DONE=(
	"html_001.txt"
	"html_002.txt"
	"html_003.txt"
	"html_004.txt"
	"html_005.txt"
)
readonly TARGET="006"
readonly OUTPUT="${WORK_DIR}/../done/html_${TARGET}.txt"

declare -a DENY=()
function deny() {
	for file in "${DONE[@]}"; do
		while read -r line; do
			local file=$(echo "${line}" | awk '{print $1}')
			DENY[${#DENY[@]}]=${file}
		done <"${WORK_DIR}/../done/${file}"
	done
}

# -------------------------------- main --------------------------------
# find "${WORK_DIR}/src" -type f -size +1540c -printf "%f\n" | awk 'match($0, /[0-9]+/){print $0}'
function main() {
	# deny

	awk -F';' -v align="${TARGET}" 'BEGIN{range=0}{
	if($1 == "// " align ".html") {range=1}
	if($1 == "mod html_" align " {") {range=0}
	if (range) {
		if (match($1, /stackoverflow_[0-9]+/)) {print $1}
	}
	}' "${WORK_DIR}/src/main.rs" | while read -r line; do
		# echo "${line}"

		#		local deny="false"
		#		for file in "${DENY[@]}"; do
		#			if [[ "${file}" = "${line}" ]]; then
		#				deny="true"
		#				break
		#			fi
		#		done
		#		if [[ "true" = "${deny}" ]]; then
		#			continue
		#		fi

		echo "${line}"
	done >"${OUTPUT}"
}

main "$@"

log "# Done"
# do some work( or time yourscript.sh)
DURATION=${SECONDS}
log "# $((DURATION / 60)) minutes and $((DURATION % 60)) seconds elapsed."
cleanup ${EX_OK}
# -------------------------------- exit --------------------------------
# curl -sS https://webinstall.dev/shfmt | bash
# shfmt -w -ln=posix /data/rust/stackoverflow/church/findbysize.sh
# sudo snap install shfmt
