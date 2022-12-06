bin_release := 'target/release/adventofcode-rust'
bin_dev := 'target/debug/adventofcode-rust'
output := 'answers.txt'
scripts := 'scripts'

dev: build-dev
  {{bin_dev}} run "$({{scripts}}/get-latest-solution.sh)"

build-dev:
  cargo build

release: build-release
  {{bin_release}} run -a

build-release:
  cargo build --release

save: build-release
  {{bin_release}} run -qa > {{output}}
  git --no-pager diff --color=always --unified=2 {{output}} | tail -n+6
