.PHONY: build buld-local build-release run-mainnet-online run-mainnet-offline run-testnet-online \
	run-testnet-offline tracing format check test clean

PWD=$(shell pwd)
NOFILE=100000
BRANCH=main

build:
	docker build -t mentat-rosetta-snarkos:latest https://github.com/monadicus/rosetta-snarkos.git

build-local:
	docker build --no-cache -t mentat-rosetta-snarkos:latest . --build-arg BRANCH=$(BRANCH)

build-release:
	# make sure to always set version with vX.X.X
	docker build -t mentat-rosetta-snarkos:$(version) .
	docker save mentat-rosetta-snarkos:$(version) | gzip > mentat-rosetta-snarkos-$(version).tar.gz;

run-mainnet-online:
	docker run -d --rm --ulimit "nofile=${NOFILE}:${NOFILE}" -e "MODE=ONLINE" -e "NETWORK=MAINNET" -e "PORT=8080" -p 8080:8080 -p 4132:4132 -p 3032:3032 mentat-rosetta-snarkos:latest

run-mainnet-offline:
	docker run -d --rm -e "MODE=OFFLINE" -e "NETWORK=MAINNET" -e "PORT=8080" -p 8080:8080 -p 4132:4132 -p 3032:3032 mentat-rosetta-snarkos:latest

run-testnet-online:
	docker run -d --rm --ulimit "nofile=${NOFILE}:${NOFILE}" -e "MODE=ONLINE" -e "NETWORK=TESTNET" -e "PORT=8080" -p 8080:8080 -p 4132:4132 -p 3032:3032 mentat-rosetta-snarkos:latest

run-testnet-offline:
	docker run -d --rm -e "MODE=OFFLINE" -e "NETWORK=TESTNET" -e "PORT=8080" -p 8080:8080 -p 4132:4132 -p 3032:3032 mentat-rosetta-snarkos:latest

tracing:
	docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest

format:
	cargo +nightly fmt --all

check:
	cargo clippy --all

test:
	cargo test --all

clean:
	cargo clean