find ./ -type f -name "*.pdf" -printf "%f\n" | sort | while read -r file; do
	# echo "${file}"
	new=$(echo "${file// /_}")
	echo "${new}"
	# git mv -f "${file}" "${new}"
	# mv -f "${file}" "${new}"
done
