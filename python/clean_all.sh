#!/bin/bash
#set -x
SECONDS=0
EX_OK=0
#EX_USAGE=64

set -o errexit
set -o nounset
set -o pipefail

# get absolute path of a path name
function abspath() {
	[[ -n "$1" ]] && (cd "$1" 2>/dev/null && pwd)
}

WORK_DIR=$(abspath "$(dirname "$0")")
echo "# WORK_DIR = ${WORK_DIR}"

function main() {
	find "${WORK_DIR}" -type f -name "Cargo.toml" | sort | while read -r file; do
		dir=$(dirname "${file}")
		pushd "${dir}" >/dev/null
		echo "clean ${dir}"
		cargo clean
		popd >/dev/null
	done

	rm -fr "${WORK_DIR}/church/r20/a"
}

main "$@"

echo "# Done"
# do some work( or time yourscript.sh)
duration=${SECONDS}
echo "# $((duration / 60)) minutes and $((duration % 60)) seconds elapsed."
exit ${EX_OK}
