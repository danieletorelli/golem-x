#!/bin/bash

set -euo pipefail

GOLEM_COMMAND="golem-cli"

function build() {
  set +u
  if [ -n "${SKIP_BUILD}" ]; then
    echo "Skipping build"
  else
    cargo make regenerate-stubs
    cargo make release-build-flow
  fi
  set -u
}

function update_component() {
  echo "Updating component: '${1?}' with wasm file '${2?}.wasm'"
  ${GOLEM_COMMAND} component update --non-interactive --component-name=${1?} target/wasm32-wasip1/release/${2?}.wasm
}

#function update_worker() {
#  echo "Updating worker: '${2?}' of component '${1?}' to version '${3?}'"
#  ${GOLEM_COMMAND} worker update --project-name=golem-x --component-name=${1?} --worker-name=${2?} --mode=auto --target-version=${3?}
#}
#
#function update_many_workers() {
#  echo "Updating all workers of component: '${1?}' to version '${2?}'"
#  ${GOLEM_COMMAND} worker update-many --project-name=golem-x --component-name=${1?} --mode=auto --target-version=${2?}
#}

function update_workers() {
  ${GOLEM_COMMAND} component try-update-workers --component-name=${1?}
}

function update_router() {
  update_component router router_composed
  update_workers router
}

function update_user_manager() {
  update_component user_manager user_management
  update_workers user_manager
}

function update_tweet_manager() {
  update_component tweet_manager tweet_management
  update_workers tweet_manager
}

function update_timeline_manager() {
  update_component timeline_manager timeline_management
  update_workers timeline_manager
}

function get-component-id() {
  ${GOLEM_COMMAND} component list --component-name=${1?} | grep "${1?}" | awk -F '|' '{print $2}' | tail -n 1 | tr -dc '[:print:]' | tr -d '[:space:]' | sed 's/\[[0-9;]*[mK]//g' | sed 's/urn:component://'
}

function get-component-version() {
  ${GOLEM_COMMAND} component list --component-name=${1?} | grep "${1?}" | awk -F '|' '{print $6}' | tail -n 1 | tr -dc '[:print:]' | tr -d '[:space:]' | sed 's/\[[0-9;]*[mK]//g'
}

function get-worker-version() {
  ${GOLEM_COMMAND} worker list --component-name=${1?} | (grep "${1?}" || true) | awk -F '|' '{print $5}' | tr -dc '[:print:]' | tr -d '[:space:]' | sed 's/\[[0-9;]*[mK]//g'
}

function update-api() {
  ${GOLEM_COMMAND} api-deployment delete golem-x.localhost:9006 || true
  ${GOLEM_COMMAND} api-definition delete --id=golem-x-v1 --version=0.0.1 || true

  ROUTER_COMPONENT_ID=$(get-component-id router)
  ROUTER_WORKER_VERSION=$(get-worker-version router)

  if [ -z "${ROUTER_WORKER_VERSION}" ]; then
    create-router-worker
    ROUTER_COMPONENT_ID=$(get-component-id router)
    ROUTER_WORKER_VERSION=$(get-worker-version router)
  fi

  sed_backup=""
  if [[ "$OSTYPE" == "darwin"* ]]; then
    sed_backup="''"
  fi

  sed -i $sed_backup "s/\"componentId\": \"[0-9a-fA-F\-]\{36\}\"/\"componentId\": \"${ROUTER_COMPONENT_ID}\"/g" api-definition.json
  sed -i $sed_backup "s/\"version\": [0-9]/\"version\": ${ROUTER_WORKER_VERSION}/g" api-definition.json

  ${GOLEM_COMMAND} api-definition add api-definition.json
  ${GOLEM_COMMAND} api-deployment deploy --definition=golem-x-v1/0.0.1 --host=localhost:9006 --subdomain=golem-x
}

function create-router-worker {
  USER_MANAGER_COMPONENT_ID=$(get-component-id user_manager)
  TWEET_MANAGER_COMPONENT_ID=$(get-component-id tweet_manager)
  TIMELINE_MANAGER_COMPONENT_ID=$(get-component-id timeline_manager)

  echo "Creating router worker (user_manager: ${USER_MANAGER_COMPONENT_ID}, tweet_manager: ${TWEET_MANAGER_COMPONENT_ID}, timeline_manager: ${TIMELINE_MANAGER_COMPONENT_ID})"

  ${GOLEM_COMMAND} worker add --component-name=router --worker-name=router \
    --env=USER_MANAGER_COMPONENT_ID=${USER_MANAGER_COMPONENT_ID} \
    --env=TWEET_MANAGER_COMPONENT_ID=${TWEET_MANAGER_COMPONENT_ID} \
    --env=TIMELINE_MANAGER_COMPONENT_ID=${TIMELINE_MANAGER_COMPONENT_ID}
}

if [ $# -eq 0 ]; then
  build
  update_router
  update_user_manager
  update_tweet_manager
  update_timeline_manager
  update-api
else
  for arg in "$@"; do
    case $arg in
      build)
        build
        ;;
      router)
        update_router
        ;;
      user_manager)
        update_user_manager
        ;;
      tweet_manager)
        update_tweet_manager
        ;;
      timeline_manager)
        update_timeline_manager
        ;;
      api)
        update-api
        ;;
      *)
        echo "Invalid argument: $arg"
        exit 1
        ;;
    esac
  done
fi