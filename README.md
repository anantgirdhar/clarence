# clarence (Command Line Academic ReferENCE manager)

A simple tool to manage a reading list / academic references offline.

## Dependencies

- sed
- awk
- cut
- jq
- nl (to number lines and present menus)
- curl (to query crossref)
- iconv (to remove diacritics from entry keys)
- pdfinfo
- pdftotext
- fzf
- bat (for syntax highlighting)
- pandoc-citeproc

## Installation

Run

    sudo make install

If you want to run without sudo, change the PREFIX variable in the Makefile to
somewhere your user can write to.

Alternatively, copy the scripts to somewhere in your `$PATH`.

## Configuration

The configuration file needs to be placed at `$XDG_CONFIG_HOME/clarence/rc`.
The paths in this file can be customized according to your setup. A sample
config file is provided in the source folder (sample_config).

## Inserting Citations

If you want to be able to cite while writing in vim (that's what I work with),
you can call clarence's fuzzy search feature within vim to pick the citations
you'd like to insert. To help with note taking in markdown,
`editor/markdown.vim` file has been provided. Either put the contents of this
file into your `.vimrc` or place the file in your vim configuration at
`vim/after/ftplugin/`.
