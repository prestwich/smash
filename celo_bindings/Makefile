# Go parameters
GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean
GOGET=$(GOCMD) get
BIN=bin
LIB_NAME=callcelo
LIBARY_FULL_NAME=lib$(LIB_NAME).so
ARCHIVE_FULL_NAME=lib$(LIB_NAME).a
LIB_DIR=./lib
SRC=.
CXX=gcc
CXX_FLAGS := -L$(LIB_DIR) -l$(LIB_NAME) -I$(LIB_DIR)

all: clean build

static: clean build_static

external: clean build_external

static_external: clean build_static_external

build:
		CGO_ENABLED=1 $(GOBUILD) -o $(LIB_DIR)/$(LIBARY_FULL_NAME) -buildmode=c-shared $(SRC)/lib.go
build_static:
		CGO_ENABLED=1 $(GOBUILD) -o $(LIB_DIR)/$(ARCHIVE_FULL_NAME) -buildmode=c-archive $(SRC)/lib.go
build_external:
		CGO_ENABLED=1 $(GOBUILD) -o $(OUT_DIR)/$(LIBARY_FULL_NAME) -buildmode=c-shared $(SRC)/lib.go
build_static_external:
		CGO_ENABLED=1 $(GOBUILD) -o $(OUT_DIR)/$(ARCHIVE_FULL_NAME) -buildmode=c-archive $(SRC)/lib.go
clean:
		$(GOCLEAN)
		rm -f $(LIB_DIR)/*