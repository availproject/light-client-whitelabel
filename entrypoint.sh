#!/bin/sh

echo "Light Client version: $(./avail-light-whitelabel --version)"

echo ""

echo "## Running Light Client"

./avail-light-whitelabel "$@"

