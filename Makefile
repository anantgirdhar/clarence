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

debug: $(RMGR)/* $(UTILS)
	mkdir -p $(PREFIX)/bin
	ln -s `pwd`/$(RMGR) $(PREFIX)/bin
	for f in $(UTILS); do \
		ln -s `pwd`/$$f $(PREFIX)/bin/; \
	done
	ln -s `pwd`/$(MAIN) $(PREFIX)/bin

cleandebug:
	rm -f $(PREFIX)/bin/$(MAIN)
	rm -f $(PREFIX)/bin/$(RMGR)
	for f in $(UTILS); do \
		rm -f $(PREFIX)/bin/$$(basename $$f); \
	done

.PHONY: install uninstall debug cleandebug
