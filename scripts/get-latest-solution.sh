#!/bin/bash

set -euo pipefail

/usr/bin/ls -t src/year*/*.rs | \
    head -n1 | \
    sed 's/day/*/' | \
    sed 's/[^0-9*]//g' | \
    sed 's/*/./'

