bin_release := 'target/release/adventofcode-rust'
bin_dev := 'target/debug/adventofcode-rust'
output := 'answers.txt'
scripts := 'scripts'
latest := `scripts/get-latest-solution.sh`

dev: build-dev
  {{bin_dev}} run '{{latest}}'

dev-all: build-dev
  {{bin_dev}} run '*'

build-dev:
  cargo build

release: build-release
  {{bin_release}} run '{{latest}}'

release-all: build-release
  {{bin_release}} run '*'

build-release:
  cargo build --release

save: build-release
  {{bin_release}} run --quiet '*' > {{output}}
  git --no-pager diff --color=always --unified=2 {{output}} | tail -n+6

bench: build-release
  {{bin_release}} run '{{latest}}:real' --min-runs 5 --min-duration-s 1 --color=always

bench-all: build-release
  {{bin_release}} run '*:real' --min-runs 5 --min-duration-s 1 --color=always | \
    {{scripts}}/tee-uncolored.sh bench.txt

flamegraph: build-release
  cargo flamegraph -- run '{{latest}}:real' --min-runs 5 --min-duration-s 1 --color=always
  perf script -F +pid > perf.txt

list: build-dev
  {{bin_dev}} list
