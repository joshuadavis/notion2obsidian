# notion2obsidian

`notion2obsidian` converts Notion exports into Obsidian markdown files/folders.

# Features

Details about the conversion that `notion2obsidian` does...

## Rename Folders and Notes

* **Removes the UUIDs** that Notion adds to folders and to markdown files.  Internal links between markdown files use the modified folder / file names.

## Translate Links

* Internal links need to reference the renamed files and folders.
* Internal links also need to be in Obsidian Internal Link format.
* External links stay the same.

## Transform CSV Files Into Tables

* Notion exports "database" pages as CSV files.   These are translated into markdown files with a markdown table in them.

## Work in Progress

* Translate `Tags: foo, bar, baz` into Obsidian tags.
* Remove first heading if it's the same as the page name.
* Detect "link tables" and link the names to the corresponding notes.

# Installation

... to be written


# Usage

... to be written

# Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).
