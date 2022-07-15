VERSION = 0.2.0
PREFIX = /usr/local
COMPLETIONS_PREFIX=/usr/local/share/zsh/completions

MAIN = clarence
RMGR = rmgr
UTILS = utils/*

.DEFAULT: install

install: $(RMGR)/* $(UTILS)
	mkdir -p $(PREFIX)/bin
	cp -rf $(RMGR) $(PREFIX)/bin
	cp -f $(UTILS) $(PREFIX)/bin
	cp -f $(MAIN) $(PREFIX)/bin
	mkdir -p $(COMPLETIONS_PREFIX)
	cp -f `pwd`/autocompletions/zsh $(COMPLETIONS_PREFIX)/_$(MAIN)

uninstall:
	rm -f $(COMPLETIONS_PREFIX)/_$(MAIN)
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
	mkdir -p $(COMPLETIONS_PREFIX)
	ln -s `pwd`/autocompletions/zsh $(COMPLETIONS_PREFIX)/_$(MAIN)

cleandebug:
	rm -f $(COMPLETIONS_PREFIX)/_$(MAIN)
	rm -f $(PREFIX)/bin/$(MAIN)
	rm -f $(PREFIX)/bin/$(RMGR)
	for f in $(UTILS); do \
		rm -f $(PREFIX)/bin/$$(basename $$f); \
	done

.PHONY: install uninstall debug cleandebug
