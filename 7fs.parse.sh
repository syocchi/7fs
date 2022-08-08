#!/bin/bash

logging_indent=0

function logging {

  case $1 in
    verbose) echo -n "[*] ";;
    info) echo -n "[ ] ";;
    warning) echo -n "[!] ";;
    error) echo -n "<!> ";;
  esac
  printf "%${logging_indent}s"
  shift
  echo $*
}

function logging_group {
  logging $*
  logging_indent=$((logging_indent+2))
}

function logging_group_end {
  logging_indent=$((logging_indent-2))
}


declare -g buffer=""

# read_n(length, write_to)
function read_n {
  local len=$1

  local ret=${buffer:0:$len}
  buffer=${buffer:$len}

  declare -n ret_ref=$2
  ret_ref=$ret

  # logging verbose "read_n: $1 -> $ret"
}

# expect(length, expect, assert_message)
function expect {
  local length=$1
  local expected=$2
  local assert_message=$3

  read_n $length actually
  if [ "$actually" == "$expected" ]; then
    logging verbose "$assert_message succeeded"
  else
    logging error "$assert_message failed: expected [$expected], actually [$actually]"
    exit 1
  fi
}

# timestamp(write_to)
function timestamp {
  read_n 16 $1
}

function zt_string {
  local ret=""
  local c=""
  read_n 2 c
  ret="$ret$c"
  while [ "$c" != "00" ]; do
    read_n 2 c
    ret=$ret$c
  done
  ret=${ret:0:${#ret}-2}

  ret=`echo $ret | xxd -r -p`

  local -n ret_ref=$1
  ret_ref=$ret
}

function header {
  expect 8 ff767f73 "7fs signature verification"
  expect 8 00010001 "7fs version check"
  expect 16 0000000000000000 "7fs reserved"
}
function u32_data {
  read_n 8 length
  length=$((16#$length))
  length=$((length*2))
  read_n $length $1
}
function item {
  timestamp timestamp
  zt_string name
  read_n 2 kind
  case $kind in
    01)
      logging info "File: \"$name\" at $timestamp"
      u32_data data
      # xxd -r -p <(echo $data) | xxd
      ;;
    02)
      logging_group info "Folder: \"$name\" at $timestamp"
      while [ "${buffer::2}" != "ff" ]; do
        item
      done
      read_n 2 _
      logging_group_end;;
    03)
      zt_string url
      logging info "URL File: \"$name\" -> \"$url\" at $timestamp";;
    04)
      zt_string target
      logging info "Shortcut: \"$name\" -> \"$target\" at $timestamp";;
    05)
      logging info "Zip Archive: \"$name\" at $timestamp"
      u32_data data
      xxd -r -p <(echo $data) | xxd;;

    06)
      logging info "Tar Archive: \"$name\" at $timestamp"
      u32_data data
      xxd -r -p <(echo $data) | xxd;;
  esac
}

function parse {
  buffer=$(xxd -p | tr -d '\n')

  header
  item
}

cat test.7fs | parse