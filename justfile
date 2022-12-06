bin_release := 'target/release/adventofcode-rust'
bin_dev := 'target/debug/adventofcode-rust'
output := 'answers.txt'
scripts := 'scripts'

dev: build-dev
  {{bin_dev}} run "$({{scripts}}/get-latest-solution.sh)"

dev-all: build-dev
  {{bin_dev}} run --all

build-dev:
  cargo build

release: build-release
  {{bin_release}} run "$({{scripts}}/get-latest-solution.sh)"

release-all: build-release
  {{bin_release}} run --all

build-release:
  cargo build --release

save: build-release
  {{bin_release}} run --quiet --all > {{output}}
  git --no-pager diff --color=always --unified=2 {{output}} | tail -n+6

bench: build-release
  {{bin_release}} run --all --min-runs 5 --min-duration-s 1 --color=always | \
    {{scripts}}/tee-uncolored.sh bench.txt
