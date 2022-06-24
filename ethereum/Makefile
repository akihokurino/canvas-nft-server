MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

truffle-init:
	./node_modules/.bin/truffle init

truffle-compile:
	./node_modules/.bin/truffle compile

truffle-develop:
	./node_modules/.bin/truffle develop

truffle-migrate-rinkeby-net:
	./node_modules/.bin/truffle migrate --reset --network rinkeby

truffle-console-rinkeby-net:
	./node_modules/.bin/truffle console --network rinkeby	