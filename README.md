# clarence (Command Line Academic ReferENCE manager)

A simple tool to manage a reading list / academic references offline.

## Dependencies

sed
awk
cut
jq
curl
iconv
pdfinfo
pdftotext
pandoc-citeproc

## Installation

Run

    sudo make install

If you want to run without sudo, change the PREFIX variable in the Makefile to
somewhere your user can write to.

Alternatively, copy the scripts to somewhere in your $PATH.

## Configuration

The configuration file needs to be placed at $XDG_CONFIG_HOME/clarence/rc.
The paths in this file can be customized according to your setup. A sample
config file is provided in the source folder (sample_config).
