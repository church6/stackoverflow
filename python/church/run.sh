#!/bin/bash
# @generator          :  template.sh
# @filename           :  run.sh
# @author             :  Copyright (C) Church.ZHONG
# @date               :  Wed Nov 16 03:24:52 PM HKT 2022
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

function tool() {
	file="$*"
    # echo "${file}"
    black --line-length 128 --skip-string-normalization --skip-magic-trailing-comma "${file}" 2> /dev/null
    flake8 --max-line-length 256 --indent-size 4 "${file}" > /dev/null
    pylint --max-line-length=256 --indent-after-paren=4 "${file}" | awk '!/^\\s*\$/ && !/Your code has been rated at/ && !/------------------------------------------------------------------/{print \$0}'
}

function fmt() {
	find "${WORK_DIR}" -type f -name "stackoverflow_*.py" | while read -r file; do
    # echo "${file}"
    tool "${file}"
	done
}
# -------------------------------- main --------------------------------
function main() {
	# clear
	git status --short | awk '$1 ~ /^M$/{$1="";print $0}' | while read -r file; do
		if [[ "${file}" == *".py" ]]; then
	    tool "${file}"
		fi
	done

	pushd "${WORK_DIR}" || {
		echo "${WORK_DIR}: No such file or directory"
		exit ${EX_USAGE}
	}

	readonly TEMP="/tmp/tmp.stackoverflow.python"
	python3 "${WORK_DIR}/main.py" -i "$(cat ${TEMP})"

	popd "${WORK_DIR}"
}

main "$@"

log "# Done"
# do some work( or time yourscript.sh)
DURATION=${SECONDS}
log "# $((DURATION / 60)) minutes and $((DURATION % 60)) seconds elapsed."
cleanup ${EX_OK}
# -------------------------------- exit --------------------------------
# curl -sS https://webinstall.dev/shfmt | bash
# shfmt -w -ln=posix /data/rust/stackoverflow/church/run.sh
# sudo snap install shfmt
