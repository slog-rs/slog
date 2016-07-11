PKG_NAME=$(shell grep name Cargo.toml | head -n 1 | awk -F \" '{print $$2}')
DOCS_DEFAULT_MODULE=$(PKG_NAME)
ifeq (, $(shell which cargo-check 2> /dev/null))
DEFAULT_TARGET=build
else
DEFAULT_TARGET=build
endif

default: $(DEFAULT_TARGET)

ALL_TARGETS += build $(EXAMPLES) test doc extra
ifneq ($(RELEASE),)
$(info RELEASE BUILD: $(PKG_NAME))
CARGO_FLAGS += --release
ALL_TARGETS += bench
else
$(info DEBUG BUILD: $(PKG_NAME); use `RELEASE=true make [args]` for release build)
endif

EXAMPLES = $(shell cd examples 2>/dev/null && ls *.rs 2>/dev/null | sed -e 's/.rs$$//g' )
EXTRAS = $(shell cd extra 2>/dev/null && ls 2>/dev/null)

all: $(ALL_TARGETS)

.PHONY: run test build doc clean clippy
run test build clean:
	cargo $@ $(CARGO_FLAGS)

test-all:
	cargo test $(CARGO_FLAGS)
	cd "extra/term"; cargo test $(CARGO_FLAGS)
	cd "extra/serde"; cargo test $(CARGO_FLAGS)
	cd "extra/json"; cargo test $(CARGO_FLAGS)
	cd "extra/bunyan"; cargo test $(CARGO_FLAGS)

check:
	$(info Running check; use `make build` to actually build)
	cargo $@ $(CARGO_FLAGS)

clippy:
	cargo build --features clippy

.PHONY: bench
bench:
	cargo $@ $(filter-out --release,$(CARGO_FLAGS))

.PHONY: travistest
travistest: test-all

.PHONY: longtest
longtest:
	@echo "Running longtest. Press Ctrl+C to stop at any time"
	@sleep 2
	@i=0; while i=$$((i + 1)) && echo "Iteration $$i" && make test ; do :; done

.PHONY: $(EXAMPLES)
$(EXAMPLES):
	cargo build --example $@ $(CARGO_FLAGS)

.PHONY: extra
extra: $(EXTRAS)

.PHONY: $(EXTRAS)
$(EXTRAS):
	cd "extra/$@"; cargo build $(CARGO_FLAGS)
	cd "extra/$@"; cargo test $(CARGO_FLAGS)

.PHONY: doc
doc: FORCE
	cargo doc -p slog
	cd "extra/term"; cargo doc -p slog-term
	cd "extra/serde"; cargo doc -p slog-serde
	cd "extra/json"; cargo doc -p slog-json
	cd "extra/bunyan"; cargo doc -p slog-bunyan

.PHONY: publishdoc
publishdoc:
	rm -rf target/doc
	make doc
	echo '<meta http-equiv="refresh" content="0;url='${DOCS_DEFAULT_MODULE}'/index.html">' > target/doc/index.html
	ghp-import -n target/doc
	git push -f origin gh-pages

.PHONY: docview
docview: doc
	xdg-open target/doc/$(PKG_NAME)/index.html

.PHONY: FORCE
FORCE:
