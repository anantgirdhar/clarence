VERSION = 0.1
PREFIX = /usr/local

SRC = rmgr_*
LOCAL_UTILS = bib2key bib2type doi2bib pdf2doi
CROSSREF_UTILS = querycr

.DEFAULT: install

install: install_src install_local_utils install_crossref_utils

install_src: $(SRC)
	mkdir -p $(PREFIX)/bin
	cp -f $(SRC) $(PREFIX)/bin

install_local_utils: $(LOCAL_UTILS)
	mkdir -p $(PREFIX)/bin
	cp -f $(LOCAL_UTILS) $(PREFIX)/bin

install_crossref_utils: $(CROSSREF_UTILS)
	mkdir -p $(PREFIX)/bin
	cp -f $(CROSSREF_UTILS) $(PREFIX)/bin

uninstall:
	rm $(PREFIX)/bin/$(SRC)
	for f in $(LOCAL_UTILS); do \
		rm $(PREFIX)/bin/$$f ; \
	done
	for f in $(CROSSREF_UTILS); do \
		rm $(PREFIX)/bin/$$f ; \
	done

.PHONY: \
	install_src \
	install_local_utils \
	install_crossref_utils \
	install \
	uninstall
