# Flexiblock
Flexiblock aims to be a messy, overengineered, feature-creeped, and generally super cool Minecraft clone.

## Installer
To create a Windows installer of Flexiblock, you must be running Windows and have Wix installed.
You can get the newest version (we use 3.*) here: [https://wixtoolset.org/releases/](https://wixtoolset.org/releases/).
Simply run `source create_installer` in Git Bash in the root directory of Flexblock (same directory as Cargo.toml) and a installer (.msi) file will be created at `target/wix/flexblock.exe`.
This installer can then be run on many 64-bit Windows machines to install Flexblock as a Windows registered program.
Note however, that the installed program must be run from the directory it is installed in or the internal path references will not work.
