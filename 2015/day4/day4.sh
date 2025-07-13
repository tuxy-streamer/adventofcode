#!/bin/bash
firstHalf() {
	file="iwrupvqb"
	for ((i = 300000; ; i++)); do
		[[ $(echo -n "$file$i" | md5sum) == 00000* ]] && echo $i && break
	done
}

secondHalf() {
	file="iwrupvqb"
	for ((i = 9900000; ; i++)); do
		[[ $(echo -n "$file$i" | md5sum) == 000000* ]] && echo $i && break
	done
}

run() {
	min_positive_number=$(firstHalf)
	echo "First Half (min_positive_number) $min_positive_number"
	min_positive_number=$(secondHalf)
	echo "Second Half (min_positive_number) $min_positive_number"
}
