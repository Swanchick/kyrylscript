#!/bin/bash

echo "Initializing env variables"

source $HOME/.cargo/env
source $HOME/export-esp.sh

echo "Env vars have been loaded"

cargo run
