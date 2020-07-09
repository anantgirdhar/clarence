# refmgr - a reference manager

VERSION = 0.1
PREFIX = /usr/local

SRC = crossref notes

install:
	mkdir -p ${PREFIX}/bin
	cp -f ${SRC} ${PREFIX}/bin

uninstall:
	rm -rf ${PREFIX}/bin/refmgr

.PHONY: install
