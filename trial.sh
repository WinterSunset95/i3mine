#!/bin/bash
#
#
#

get_term_size() {
	read -r LINES COLUMNS < <(stty size)
}
get_term_size
trap 'get_term_size' WINCH
printf '\e[%sH' "$LINES"
