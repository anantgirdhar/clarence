VERSION = 1.0
PREFIX = /usr/local

MAIN = clarence
RMGR = rmgr
UTILS = utils/*

.DEFAULT: install

install: $(RMGR)/* $(UTILS)
	mkdir -p $(PREFIX)/bin
	cp -rf $(RMGR) $(PREFIX)/bin
	cp -f $(UTILS) $(PREFIX)/bin
	cp -f $(MAIN) $(PREFIX)/bin

uninstall:
	rm -f $(PREFIX)/bin/$(MAIN)
	rm -rf $(PREFIX)/bin/$(RMGR)
	for f in $(UTILS); do \
		rm -f $(PREFIX)/bin/$$(basename $$f); \
	done

.PHONY: install uninstall
