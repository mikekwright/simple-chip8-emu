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
