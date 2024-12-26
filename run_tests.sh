#!/bin/bash

set -euo pipefail
source api_calls.sh

[[ "${DEBUG:-}" == "true" ]] && set -x

compare_json "$(get_profile daniele)"       '{"status":200,"user":{"username":"daniele","followers":[],"followings":[]}}'
compare_json "$(get_profile john)"          '{"status":200,"user":{"username":"john","followers":[],"followings":[]}}'

compare_json  "$(tweet daniele Ciao)"       '{"status":201,"tweet":{"content":"Ciao","timestamp":0}}'
compare_json  "$(get_tweets daniele)"       '{"status":200,"tweets":[{"content":"Ciao","timestamp":0}]}'

compare_json  "$(follow daniele john)"      '{"status":201,"followed":true}'
compare_json  "$(get_profile daniele)"      '{"status":200,"user":{"username":"daniele","followers":[],"followings":["john"]}}'
compare_json  "$(get_profile john)"         '{"status":200,"user":{"username":"john","followers":["daniele"],"followings":[]}}'

compare_json  "$(tweet john Hello)"         '{"status":201,"tweet":{"content":"Hello","timestamp":0}}'
compare_json  "$(tweet john Heeey)"         '{"status":201,"tweet":{"content":"Heeey","timestamp":0}}'
compare_json  "$(get_timeline daniele)"     '{"status":200,"tweets":[{"author":"john","content":"Heeey","timestamp":0},{"author":"john","content":"Hello","timestamp":0}]}'
compare_json  "$(tweet john 'Good day')"    '{"status":201,"tweet":{"content":"Good day","timestamp":0}}'
compare_json  "$(get_timeline daniele)"     '{"status":200,"tweets":[{"author":"john","content":"Good day","timestamp":0},{"author":"john","content":"Heeey","timestamp":0},{"author":"john","content":"Hello","timestamp":0}]}'

compare_json  "$(follow bob daniele)"       '{"status":201,"followed":true}'
compare_json  "$(follow bob john)"          '{"status":201,"followed":true}'
compare_json  "$(get_profile bob)"          '{"status":200,"user":{"username":"bob","followers":[],"followings":["daniele","john"]}}'
compare_json  "$(get_timeline bob)"         '{"status":200,"tweets":[{"author":"daniele","content":"Ciao","timestamp":0},{"author":"john","content":"Good day","timestamp":0},{"author":"john","content":"Heeey","timestamp":0},{"author":"john","content":"Hello","timestamp":0}]}'

compare_json  "$(unfollow daniele john)"    '{"status":200,"unfollowed":true}'
compare_json  "$(get_profile daniele)"      '{"status":200,"user":{"username":"daniele","followers":["bob"],"followings":[]}}'
compare_json  "$(get_profile john)"         '{"status":200,"user":{"username":"john","followers":["bob"],"followings":[]}}'

compare_json  "$(tweet bob Yo)"             '{"status":201,"tweet":{"content":"Yo","timestamp":0}}'
compare_json  "$(get_tweets bob)"           '{"status":200,"tweets":[{"content":"Yo","timestamp":0}]}'
compare_json  "$(get_timeline daniele)"     '{"status":200,"tweets":[]}'

compare_json  "$(unfollow bob daniele)"     '{"status":200,"unfollowed":true}'
compare_json  "$(get_profile bob)"          '{"status":200,"user":{"username":"bob","followers":[],"followings":["john"]}}'
compare_json  "$(get_profile daniele)"      '{"status":200,"user":{"username":"daniele","followers":[],"followings":[]}}'
compare_json  "$(get_timeline bob)"         '{"status":200,"tweets":[{"author":"john","content":"Good day","timestamp":0},{"author":"john","content":"Heeey","timestamp":0},{"author":"john","content":"Hello","timestamp":0}]}'