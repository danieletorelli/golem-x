#!/bin/bash

set -euo pipefail

GOLEM_API_HOST="http://golem-x.localhost:9006"

function get_profile() {
  curl -Ss -H "Accept: application/json" "${GOLEM_API_HOST}/users/${1?}"
}

function follow() {
  TARGET="${2?}"
  curl -Ss -H "Accept: application/json" -X POST "${GOLEM_API_HOST}/users/${1?}/follows" -d "{\"user\":\"${TARGET}\"}" || print_oplog "user-${1?}"
}

function unfollow() {
  TARGET="${2?}"
  curl -Ss -H "Accept: application/json" -X DELETE "${GOLEM_API_HOST}/users/${1?}/follows" -d "{\"user\":\"${TARGET}\"}" || print_oplog "user-${1?}"
}

function tweet() {
  CONTENT="${2?}"
  curl -Ss -H "Accept: application/json" -X POST "${GOLEM_API_HOST}/users/${1?}/tweets" -d "{\"content\":\"${CONTENT}\"}" || print_oplog "user-${1?}"
}

function get_tweets() {
  curl -Ss -H "Accept: application/json" "${GOLEM_API_HOST}/users/${1?}/tweets" || print_oplog "user-${1?}"
}

function get_timeline() {
  curl -Ss -H "Accept: application/json" "${GOLEM_API_HOST}/users/${1?}/timeline" || print_oplog "user-${1?}"
}

if [ "${BASH_SOURCE[0]}" == "${0}" ]; then
  if [ $# -lt 1 ]; then
    echo "Usage: $0 <get_profile,follow,unfollow,tweet,get_tweets,get_timeline> [args...]"
    exit 1
  fi

  case ${1} in
    get_profile)
      get_profile "${2?}"
      ;;
    follow)
      follow "${2?}" "${3?}"
      ;;
    unfollow)
      unfollow "${2?}" "${3?}"
      ;;
    tweet)
      tweet "${2?}" "${3?}"
      ;;
    get_tweets)
      get_tweets "${2?}"
      ;;
    get_timeline)
      get_timeline "${2?}"
      ;;
    *)
      echo "Invalid argument: ${1}"
      exit 1
      ;;
  esac
fi