#!/bin/bash

# get absolute path of a path name
function abspath() {
	[[ -n "$1" ]] && (cd "$1" 2>/dev/null && pwd)
}

SELECTED_YN="no"
function select_yn() {
	local title="$1"
	SELECTED_YN=""
	read -r -p "Ready? ${title} [y/n]" sisyphus
	case ${sisyphus} in
	[Yy] | [Yy][Ee][Ss])
		SELECTED_YN="yes"
		;;
	[Nn] | [Nn][Oo])
		SELECTED_YN="no"
		;;
	*)
		#echo -n "Please type yes or no: "
		;;
	esac
}

function build() {
	if cargo clippy; then
		{
			echo "cargo build"
			cargo build
			cargo run
			cargo test
		} >/data/rust.log
	else
		/bin/true
		# >/data/rust.log
	fi
}

function run() {
	local ID="$1"
	local COUNT="$2"
	# ----------------------------------------------------------------
	select_yn "Run ID: ${ID}"
	if [[ "yes" = "${SELECTED_YN}" ]]; then
		while [[ "no" != "${SELECTED_YN}" ]]; do
			if [[ "yes" = "${SELECTED_YN}" ]]; then
				echo -e "\033[32m Running ID: \"${ID}\" \033[0m"
				fmt.pl -f "${WORK_DIR}/src/stackoverflow_${COUNT}_${ID}.rs"
				build "${ID}"
			fi

			echo "# help     = example ()"
			select_yn "Run ID: ${ID}"
		done
	fi
	# ----------------------------------------------------------------
}

readonly WORK_DIR=$(abspath "$(dirname "$0")")
echo "# WORK_DIR = ${WORK_DIR}"

readonly TEMP="${WORK_DIR}/tmp_stackoverflow.txt"
echo "# TEMP     = ${TEMP}"

readonly INPUT=$(python3 "${WORK_DIR}/next.py")
echo "# INPUT    = ${INPUT}"

if [[ -n "${INPUT}" ]]; then
	echo -n "${INPUT}" >"${TEMP}"

	PAGE=$(cut -f 1 <"${TEMP}")
	ID=$(cut -f 2 <"${TEMP}")
	COUNT=$(cut -f 3 <"${TEMP}")
	echo -e "# TEMP     = ${PAGE}\t${ID}\t${COUNT}"

	readonly RUST="${WORK_DIR}/src/stackoverflow_${COUNT}_${ID}.rs"
	echo "# RUST     = ${RUST}"

	# file "${RUST}"
	gedit "${RUST}"
	open "https://stackoverflow.com/questions/${ID}"
	sleep 1
	echo "# help     = example ()"
	run "${ID}" "${COUNT}"
fi

exit 0
