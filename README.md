# notion2obsidian

`notion2obsidian` converts Notion exports into Obsidian markdown files/folders.

# Features

Details about the conversion that `notion2obsidian` does...

## Unzip Notion export

Unzips the notion export file, if the input argument ends in '.zip'

## Rename Folders and Notes

* **Removes the UUIDs** that Notion adds to folders and to markdown files.  Internal links between markdown files use the modified folder / file names.
* Trims trailing spaces in folder and file names.   Windows doesn't like that.  Also, it's just annoying.
 
## Translate Links

* Internal links need to reference the renamed files and folders.
* Some (but not all) internal links in Notion are relative to the current file.
* Internal links also need to be in Obsidian Internal Link format: `[[destination|link text]]`
* Internal links to non-markdown files (e.g. images) are decoded (they're url-encoded in the Notion export).
* External links stay the same: `[link text](https://example.com)`
* Log a warning when links could not be converted.
* Handle relative internal links - these can just be converted into absolute internal links.

## Transform CSV Files Into Tables

* Notion exports "database" pages as CSV files.   These are translated into markdown files with a markdown table in them.
* Detect "link tables" and link the names to the corresponding notes (markdown link format is used).
* Empty cells are translated to a single space, so the table will be interpreted correctly.
* One empty line added after the last row, to ensure that the table is interpreted correctly.

## Other Transformations

* Remove first heading if it's the same as the page name.
* Translate `Tags: foo, bar, baz` into Obsidian tags, at the end of the page.

## Work in Progress

... TBD

# Installation

... to be written


# Usage

... to be written

# Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).
