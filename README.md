# ARM.rs

**Early WIP: Non-functional**

## Overview

This is a rusty rewrite of the [Automatic Ripping Machine](https://github.com/automatic-ripping-machine/automatic-ripping-machine).
It aims to alleviate a few issues I had with setting it up
as well as streamlining the codebase in a more structured manner.

Furthermore, it ~~has~~ will have built-in Nix/NixOS support out of the box.

The general structure should be very close to the original ARM described in the author's [Blogpost](https://b3n.org/automatic-ripping-machine/).


## Roadmap:

- [x] Basic functionality
    - [x] hook into udev
    - [x] parse config
    - [x] identify type of disc inserted
    - [x] eject disks

- [x] Core functionality (Movies)
    - [x] rip discs   (MakeMKV)
    - [x] encode rips (HandBrake)

- [ ] Core functionality (Series)
    - [ ] rip discs   (MakeMKV)
    - [ ] encode rips (HandBrake)

- [ ] Additional functionality
    - [ ] proper logging
    - [ ] send notifications (matrix/apprise/discord/etc.)
    - [ ] (webinterface)
