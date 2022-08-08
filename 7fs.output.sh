#!/bin/bash

function file_header() {
  printf "\xffv\x7fs\0\1\0\1"
  printf "\0\0\0\0\0\0\0\0"
}
function zero_terminated_string() {
  printf "$*"
  printf "\0"
}
function current_epoch_time() {
  printf "%016x" $(date "+%s") | xxd -r -p
}
function data() {
  local data=$(cat)
  printf "%08x" ${#data} | xxd -r -p
  printf "$data"
}
function item_header() {
  current_epoch_time
  zero_terminated_string $*
}
function item_file() {
  item_header $1
  printf "\1"
  data
}
function item_folder_begin() {
  item_header $1
  printf "\2"
}
function item_folder_end() {
  printf "\xff"
}
function item_url_file() {
  item_header $1
  printf "\3"
  zero_terminated_string $2
}
function item_shortcut() {
  item_header $1
  printf "\4"
  zero_terminated_string $2
}
function item_ziparchive() {
  item_header $1
  printf "\5"
  data
}
function item_tar_archive() {
  item_header $1
  printf "\6"
  data
}
function output_file() {
  file_header
  item_folder_begin "root"
    echo "uouo" | item_file "Test File"
    item_url_file "Test URL File" "http://www.google.com/"

    item_folder_begin "Test Folder"
      echo "uouo" | item_file "Test File"
    item_folder_end

    item_shortcut "Test Shortcut" "Test Folder"
    item_ziparchive "Test Zip Archive" < /dev/null
    item_tar_archive "Test Tar Archive" < /dev/null
  item_folder_end
}

output_file > test.7fs