# Makefile for hello

PROJECT  = hello
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:httpserver
# registry url for our actor
REG_PATH = v2/$(PROJECT):$(VERSION)
REG_URL  = localhost:5000/$(REG_PATH)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure localhost:5000/$(REG_PATH)

# friendly name for the actor
ACTOR_NAME = "hello"
# optional call alias for actor
ACTOR_ALIAS = "hello"

include ./actor.mk

