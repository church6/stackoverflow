#!/bin/bash

# @filename           :  /data/rust/example.sh
# @author             :  Copyright (C) Church.Zhong
# @date               :  Sat 09 Oct 2021 03:53:53 PM HKT
# @function           :  automatically do something what you want by shell.
# @see                :  GNU bash, version 5.0.17
# @require            :  OpenSSL 1.1.1f  31 Mar 2020/Ubuntu 20.04.2 LTS/
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

function err() {
	echo "# [$(date +'%Y-%m-%dT%H:%M:%S%z')]: $*" >&2
}

function log() {
	echo "$*" >&2
}

WORK_DIR=$(abspath "$(dirname "$0")")
echo "# WORK_DIR = ${WORK_DIR}"
OS_DATE_DAY=$(date +%Y-%m-%d)
echo "# OS_DATE_DAY = ${OS_DATE_DAY}"
OS_DATE_SECOND=$(date "+%Y_%m_%d_%H%M%S")
echo "# OS_DATE_SECOND = ${OS_DATE_SECOND}"
OUTPUT=${WORK_DIR}/${OS_DATE_SECOND}
OUTPUT=debug
echo "# OUTPUT = ${OUTPUT}"

declare -a names
names=()
names[${#names[@]}]="zero"

# https://doc.rust-lang.org/rust-by-example/index.html
# Introduction
names[${#names[@]}]="1. Hello World"
names[${#names[@]}]="2. Primitives"
names[${#names[@]}]="3. Custom Types"
names[${#names[@]}]="4. Variable Bindings"
names[${#names[@]}]="5. Types"
names[${#names[@]}]="6. Conversion"
names[${#names[@]}]="7. Expressions"
names[${#names[@]}]="8. Flow of Control"
names[${#names[@]}]="9. Functions"
names[${#names[@]}]="10. Modules"
names[${#names[@]}]="11. Crates"
names[${#names[@]}]="12. Cargo"
names[${#names[@]}]="13. Attributes"
names[${#names[@]}]="14. Generics"
names[${#names[@]}]="15. Scoping rules"
names[${#names[@]}]="16. Traits"
names[${#names[@]}]="17. macro_rules!"
names[${#names[@]}]="18. Error handling"
names[${#names[@]}]="19. Std library types"
names[${#names[@]}]="20. Std misc"
names[${#names[@]}]="21. Testing"
names[${#names[@]}]="22. Unsafe Operations"
names[${#names[@]}]="23. Compatibility"
names[${#names[@]}]="24. Meta"

COUNT=24

# -------------------------------- main --------------------------------
#DATE=$(date)
DATE="Tue 09 Aug 2022 02:11:43 PM HKT"

function examples() {
	for ((i = 1; i <= COUNT; i++)); do
		# echo "${i}"
		align=$(printf "%02d" "${i}")
		{
			cat <<CXX >"${OUTPUT}/r${align}/src/main.rs"
// @date: ${DATE}
// ${names[${i}]}

use std::time::Instant;
// https://stackoverflow.com/questions/38088067
macro_rules! _function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            // 'church
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

macro_rules! _trace {
    (\$x:expr) => {
        println!(
            "[{}][{}] {} (in {} [{}:{}:{}])",
            chrono::Local::now(),
            \$x,
            _function!(),
            module_path!(),
            file!(),
            line!(),
            column!()
        );
    };
}

macro_rules! _enter {
    () => {
        _trace!("enter")
    };
}

macro_rules! _leave {
    () => {
        _trace!("leave")
    };
}

mod example1 {
    pub fn test() {
        _enter!();
        _leave!();
    }
}
mod example2 {
    pub fn test() {
        _enter!();
        _leave!();
    }
}
mod example3 {
    pub fn test() {
        _enter!();
        _leave!();
    }
}
mod example4 {
    pub fn test() {
        _enter!();
        _leave!();
    }
}

fn main() {
    let start = Instant::now();
    println!("r${align} : ${names[${i}]}");
    example1::test();
    example2::test();
    example3::test();
    example4::test();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
CXX
		}
	done
}

function makefile() {
	{
		echo "# This file is generated automatically by example.sh"
		echo -e "# @date: ${DATE}\n"

		echo "# CXX = g++"
		echo "# GCCFLAGS += -W -Werror"
		echo "# CXXFLAGS = \$(GCCFLAGS)"
		# echo "# LDFLAGS += -Wl,--rpath -lm"
		echo "JAVA=/usr/lib/jvm/java-11-openjdk-amd64/bin/java"
		echo -e "\n"

		echo "TARGETS = \\"
		for ((i = 1; i <= COUNT; i++)); do
			align=$(printf "%02d" "${i}")
			echo -n " example${align}"
			if ((0 == i % 16)); then
				echo " \\"
			fi
		done

		echo -e "\n\n\nall:\$(TARGETS)\n"
		echo ".PRONY:clean"
		echo "clean:"
		echo "	@echo \"clean all\""
		echo -e "	rm -rf \$(TARGETS) *.o *.d *.a *.exe *.jar\n"

		for ((i = 1; i <= COUNT; i++)); do
			align=$(printf "%02d" "${i}")
			echo "example${align}: example${align}.kt"
			#echo "	\$(CXX) \$(CXXFLAGS) \$^ \$(LDFLAGS) -o \$@"
			echo "# kotlinc \$^ -include-runtime -d \$@.jar"
			echo "# \$(JAVA) -jar hello.jar"
			echo "	kotlinc \$^ -d \$@.jar"
			echo "	kotlin -classpath \$@.jar \$@.Example${align}Kt"
		done
	} >"${OUTPUT}/Makefile"
}

function main() {
	echo "write examples into ${OUTPUT}"
	rm -rf "${OUTPUT}"
	mkdir -p "${OUTPUT}"

	pushd "${OUTPUT}" >/dev/null
	for ((i = 1; i <= COUNT; i++)); do
		align=$(printf "%02d" "${i}")
		cargo new "r${align}" --vcs none
	done
	popd >/dev/null

	examples
	#makefile
}

main "$@"

log "# Done"
# do some work( or time yourscript.sh)
DURATION=${SECONDS}
log "# $((DURATION / 60)) minutes and $((DURATION % 60)) seconds elapsed."
exit ${EX_OK}
# -------------------------------- exit --------------------------------
