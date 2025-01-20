#!/bin/bash

set -euo pipefail

GOLEM_COMMAND="golem-cli"

[[ "$OSTYPE" == "darwin"* ]] && SED_FLAGS=(-i '') || SED_FLAGS=(-i)

function build() {
  [[ "${SKIP_BUILD:-}" == "true" ]] || ${GOLEM_COMMAND} app -b release build
}

function clean() {
  ${GOLEM_COMMAND} app clean
  cargo clean
}

function update_component() {
  echo "Updating component: '${1?}'"
  ${GOLEM_COMMAND} component update -b release -y --component-name=${1?}
}

function update_workers() {
  ${GOLEM_COMMAND} component try-update-workers --component-name=${1?}
}

function sanitize_output() {
  tail -n 1 | tr -dc '[:print:]' | tr -d '[:space:]' | sed 's/\[[0-9;]*[mK]//g'
}

function get_component_id() {
  ${GOLEM_COMMAND} component list --component-name=${1?} | grep "${1?}" | awk -F '|' '{print $2}' | sanitize_output | sed 's/urn:component://'
}

function get_component_version() {
  ${GOLEM_COMMAND} component list --component-name=${1?} | grep "${1?}" | awk -F '|' '{print $4}' | sanitize_output
}

function get_worker_version() {
  ${GOLEM_COMMAND} worker list --component-name=${1?} | (grep "${1?}" || true) | awk -F '|' '{print $4}' | sanitize_output
}

function update_api() {
  AUTH=${1:-none}
  ${GOLEM_COMMAND} api-deployment delete golem-x.localhost:9006 || true
  ${GOLEM_COMMAND} api-definition delete --id=golem-x --version=0.0.1 || true

  COMPONENT_ID=$(get_component_id golem-x)
  COMPONENT_VERSION=$(get_component_version golem-x)

  if [ "${AUTH}" == "auth" ]; then
    API_DEFINITION="auth-api-definition.yaml"
  else
    API_DEFINITION="api-definition.yaml"
  fi

  sed "${SED_FLAGS[@]}" -e "s/componentId: [0-9a-fA-F\-]\{36\}/componentId: ${COMPONENT_ID}/g" \
    -e "s/version: [0-9]\{1,\}$/version: ${COMPONENT_VERSION}/g" ${API_DEFINITION}

  ${GOLEM_COMMAND} api-definition add ${API_DEFINITION} --def-format yaml
  ${GOLEM_COMMAND} api-deployment deploy --definition=golem-x/0.0.1 --host=localhost:9006 --subdomain=golem-x
}

if [ $# -eq 0 ]; then
  build
  update_component golem-x
  update_workers golem-x
  update_api
else
  for arg in "$@"; do
    case $arg in
      build)
        build
        ;;
      clean)
        clean
        ;;
      update)
        update_component golem-x
        update_workers golem-x
        ;;
      api)
        update_api
        ;;
      '--auth')
        ;;
      *)
        echo "Invalid argument: $arg"
        exit 1
        ;;
    esac
  done
  case $1 in
    '--auth')
      build
      update_component golem-x
      update_workers golem-x
      update_api auth
      ;;
  esac
fi