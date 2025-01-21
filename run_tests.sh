#!/bin/bash

set -euo pipefail
source api_calls.sh

function compare_json() {
  ACTUAL=$(echo "${1?}" | jq -c -f filter.jq || echo "${1?}")
  EXPECTED=$(echo "${2?}" | jq -c -f filter.jq || echo "${2?}")

  diff --color=always -u <(echo $ACTUAL) <(echo $EXPECTED) | tail -n +4
}

function print_oplog() {
  [[ "${DEBUG:-}" == "true" ]] && golem-cli worker oplog --worker-name="user-${1?}" --component-name=golem-x && false
}

function print_workers() {
  [[ "${DEBUG:-}" == "true" ]] && golem-cli worker list --component-name=golem-x || true
}

[[ "${DEBUG:-}" == "true" ]] && set -x

print_workers

compare_json "$(get_profile daniele)"       '{"status":200,"user":{"username":"daniele","followers":[],"followings":[]}}' || print_oplog daniele
compare_json "$(get_profile john)"          '{"status":200,"user":{"username":"john","followers":[],"followings":[]}}' || print_oplog john

compare_json  "$(tweet daniele Ciao)"       '{"status":201,"tweet":{"content":"Ciao","timestamp":0}}' || print_oplog daniele
compare_json  "$(get_tweets daniele)"       '{"status":200,"tweets":[{"content":"Ciao","timestamp":0}]}' || print_oplog daniele

compare_json  "$(follow daniele john)"      '{"status":201,"followed":true}' || print_oplog daniele
compare_json  "$(get_profile daniele)"      '{"status":200,"user":{"username":"daniele","followers":[],"followings":["john"]}}' || print_oplog daniele
compare_json  "$(get_profile john)"         '{"status":200,"user":{"username":"john","followers":["daniele"],"followings":[]}}' || print_oplog john

compare_json  "$(tweet john Hello)"         '{"status":201,"tweet":{"content":"Hello","timestamp":0}}' || print_oplog john
compare_json  "$(tweet john Heeey)"         '{"status":201,"tweet":{"content":"Heeey","timestamp":0}}' || print_oplog john
compare_json  "$(get_timeline daniele)"     '{"status":200,"tweets":[{"author":"john","content":"Heeey","timestamp":0},{"author":"john","content":"Hello","timestamp":0}]}' || print_oplog daniele
compare_json  "$(tweet john 'Good day')"    '{"status":201,"tweet":{"content":"Good day","timestamp":0}}' || print_oplog john
compare_json  "$(get_timeline daniele)"     '{"status":200,"tweets":[{"author":"john","content":"Good day","timestamp":0},{"author":"john","content":"Heeey","timestamp":0},{"author":"john","content":"Hello","timestamp":0}]}' || print_oplog daniele

compare_json  "$(follow bob daniele)"       '{"status":201,"followed":true}' || print_oplog bob
compare_json  "$(follow bob john)"          '{"status":201,"followed":true}' || print_oplog bob
compare_json  "$(get_profile bob)"          '{"status":200,"user":{"username":"bob","followers":[],"followings":["daniele","john"]}}' || print_oplog bob
compare_json  "$(get_timeline bob)"         '{"status":200,"tweets":[{"author":"daniele","content":"Ciao","timestamp":0},{"author":"john","content":"Good day","timestamp":0},{"author":"john","content":"Heeey","timestamp":0},{"author":"john","content":"Hello","timestamp":0}]}' || print_oplog bob

compare_json  "$(unfollow daniele john)"    '{"status":200,"unfollowed":true}' || print_oplog daniele
compare_json  "$(get_profile daniele)"      '{"status":200,"user":{"username":"daniele","followers":["bob"],"followings":[]}}' || print_oplog daniele
compare_json  "$(get_profile john)"         '{"status":200,"user":{"username":"john","followers":["bob"],"followings":[]}}' || print_oplog john

compare_json  "$(tweet bob Yo)"             '{"status":201,"tweet":{"content":"Yo","timestamp":0}}' || print_oplog bob
compare_json  "$(get_tweets bob)"           '{"status":200,"tweets":[{"content":"Yo","timestamp":0}]}' || print_oplog bob
compare_json  "$(get_timeline daniele)"     '{"status":200,"tweets":[]}' || print_oplog daniele

compare_json  "$(unfollow bob daniele)"     '{"status":200,"unfollowed":true}' || print_oplog bob
compare_json  "$(get_profile bob)"          '{"status":200,"user":{"username":"bob","followers":[],"followings":["john"]}}' || print_oplog bob
compare_json  "$(get_profile daniele)"      '{"status":200,"user":{"username":"daniele","followers":[],"followings":[]}}' || print_oplog daniele
compare_json  "$(get_timeline bob)"         '{"status":200,"tweets":[{"author":"john","content":"Good day","timestamp":0},{"author":"john","content":"Heeey","timestamp":0},{"author":"john","content":"Hello","timestamp":0}]}' || print_oplog bob

print_workers