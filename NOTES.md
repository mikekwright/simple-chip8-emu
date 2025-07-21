Project Notes
===============================================

July 8th, 2025
------------------------------

Project initially setup.  When created started with a few items to be the
placeholder for hardware components.

First thing was to get the display and setup a GUI app.

Started with eframe, but had issues, so moved to Iced.  Still had issues
and found that it was a nixos problem, the fix was to include library
items for wayland and then an `LD_LIBRARY_PATH` change.  This allowed
the application to run with Iced.

July 9th, 2025
------------------------------

Spent time today just getting more familiar with the iced system and
how it approaches widgets.  Main focus is still on the app startup
and the visual component being setup.

(Also spent time getting ai instructions added, hoping it will help)

July 20th, 2025
---------------------------------------

At this time was looking over the project and the thought came to use
a console gui instead (which is a TUI).  So I used copilot to generate
the framework that allows for both a TUI and GUI.  This introduced:

* [ratatui](https://ratatui.rs/tutorials/hello-ratatui/)
* [crossterm](https://docs.rs/crossterm/latest/crossterm/)
* a cli parsing solution [clap](https://docs.rs/clap/latest/clap/)

The next thing was to figure out how to optimize the build to not include
the gui code, but still have it available. (Finding out this is a
cargo `feature flag`)
