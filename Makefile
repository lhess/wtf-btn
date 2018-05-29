MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables
MAKEFLAGS += --no-print-directory
MAKEFLAGS += --warn-undefined-variables
.DELETE_ON_ERROR :=
SHELL       := bash
.SHELLFLAGS := -euo pipefail -c
.ONESHELL:
.SUFFIXES:

TARGET  ?= arm-unknown-linux-gnueabihf

DOCKER_RUN := docker run -v $(CURDIR):/usr/local/src lhess/rust-pi0
CARGO := $(DOCKER_RUN) cargo

export ROOT := $(abspath $(CURDIR))
export PATH := $(PATH):$(ROOT)/resources/make/bin

REMOTE_HOST := 192.168.35.42
REMOTE_USER := pi

all:: image format build deploy run

image:: Dockerfile
Dockerfile::
	docker build --build-arg GCC_PKG=$(GCC_PKG) --build-arg TARGET=$(TARGET) --tag lhess/rust-pi0 .

build-debug::
	cargo build --target=$(TARGET)

build-release::
	cargo build --target=$(TARGET) --release

build:: image
	$(DOCKER_RUN) make build-release

check::
	$(CARGO) check --verbose

clean::
	$(CARGO) clean

debug::
	$(DOCKER_RUN) make build-debug

format::
	$(CARGO) fmt

lint::
	$(CARGO) +nightly clippy

deploy::
	for profile in release debug; do
		bp=target/$(TARGET)/$${profile}/wtf-btn
		if [[ -f $${bp} ]]; then
			scp $${bp} $(REMOTE_USER)@$(REMOTE_HOST):~/bin/wtf-btn
			break
		fi
	done

run::
	ssh $(REMOTE_USER)@$(REMOTE_HOST) "RUST_BACKTRACE=1 ~/bin/wtf-btn"
