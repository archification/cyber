# Cyber

Cyber is a cli program that can extract all zip files in a folder and move their files to another folder as well as keep track of the name of each archive the files came from so they might be easily removed later.

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)

## Installation

Put the exe, json, and config files in a folder together. Then run the exe.

## Usage

The program will display a menu with three options, the third just being to exit the program.

The install option will look at the config file to get paths, unarchive all zip files in the first path, and then move all the extracted files to the second path. A record of all extracted archives and their moved files will be stored in the json file.

The uninstall option will look at the json file and display a menu containing all installed mods. Selecting a mod here will cause the files associated with that archive to be deleted.

If installing a mod would cause a file to be overwritten, the user is prompted to overwrite that file.

## Contributing

Issues and PRs will be reviewed. Feedback is welcome.
