bin_release := 'target/release/adventofcode-rust'
bin_dev := 'target/debug/adventofcode-rust'
output := 'answers.txt'
scripts := 'scripts'
latest := `scripts/get-latest-solution.sh`

dev part='*': build-dev
  {{bin_dev}} run '{{latest}}:{{part}}'

dev-all part='*': build-dev
  {{bin_dev}} run '*:{{part}}'

build-dev:
  cargo build

release part='*': build-release
  {{bin_release}} run '{{latest}}:{{part}}'

release-all part='*': build-release
  {{bin_release}} run '*:{{part}}'

build-release:
  cargo build --release

save: build-release
  {{bin_release}} run --quiet '*' > {{output}}
  git --no-pager diff --color=always --unified=2 {{output}} | tail -n+6

bench part='real': build-release
  {{bin_release}} run '{{latest}}:{{part}}' --min-runs 5 --min-duration-s 1 --color=always

bench-all part='real': build-release
  {{bin_release}} run '*:{{part}}' --min-runs 5 --min-duration-s 1 --color=always | \
    {{scripts}}/tee-uncolored.sh bench.txt

flamegraph part='*': build-release
  cargo flamegraph -- run '{{latest}}:{{part}}' --min-runs 5 --min-duration-s 1 --color=always
  perf script -F +pid > perf.txt

list: build-dev
  {{bin_dev}} list
