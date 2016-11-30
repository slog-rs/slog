PKG_NAME=$(shell grep name Cargo.toml | head -n 1 | awk -F \" '{print $$2}')
DOCS_DEFAULT_MODULE=$(subst -,_,$(PKG_NAME))
ifeq (, $(shell which cargo-check 2> /dev/null))
DEFAULT_TARGET=build
else
DEFAULT_TARGET=build
endif

default: $(DEFAULT_TARGET)

CARGO_FLAGS += -v

ALL_TARGETS += build $(EXAMPLES) test doc crates
ifneq ($(RELEASE),)
$(info RELEASE BUILD: $(PKG_NAME))
CARGO_FLAGS += --release
else
$(info DEBUG BUILD: $(PKG_NAME); use `RELEASE=true make [args]` for release build)
endif

EXAMPLES = $(shell cd examples 2>/dev/null && ls *.rs 2>/dev/null | sed -e 's/.rs$$//g' )
CRATES = $(shell cd crates 2>/dev/null && ls 2>/dev/null)

all: $(ALL_TARGETS)

.PHONY: run test build doc clean clippy
run test build clean:
	cargo $@ $(CARGO_FLAGS)

test-all:
	cargo test $(CARGO_FLAGS)
	cd "crates/serde"; cargo test $(CARGO_FLAGS)
	cd "crates/json"; cargo test $(CARGO_FLAGS)
	cd "crates/bunyan"; cargo test $(CARGO_FLAGS)
	cd "crates/stdlog"; cargo test $(CARGO_FLAGS)
	cd "crates/scope"; cargo test $(CARGO_FLAGS)
	cd "crates/example-lib"; cargo test $(CARGO_FLAGS)
	cd "crates/nursery"; cargo test $(CARGO_FLAGS)

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

.PHONY: crates
crates: $(CRATES)

.PHONY: $(CRATES)
$(CRATES):
	cd "crates/$@"; cargo build $(CARGO_FLAGS)
	cd "crates/$@"; cargo test $(CARGO_FLAGS)

.PHONY: doc
doc: FORCE
	cargo doc
	cd "crates/serde"; cargo doc -p slog-serde
	cd "crates/json"; cargo doc -p slog-json
	cd "crates/bunyan"; cargo doc -p slog-bunyan
	cd "crates/stdlog"; cargo doc -p slog-stdlog
	cd "crates/scope"; cargo doc -p slog-scope
	cd "crates/example-lib"; cargo doc -p slog-example-lib
	cd "crates/nursery"; cargo doc -p slog-nursery

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
