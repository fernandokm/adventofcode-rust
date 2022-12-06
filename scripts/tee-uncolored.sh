#!/bin/bash

set -euo pipefail

tee >(sed --unbuffered 's/\x1B\[[0-9;]\{1,\}[A-Za-z]//g' > "$1")

