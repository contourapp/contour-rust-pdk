# contour-rust-pdk

This package is used by plugins. It contains the definitions for the host functions accessible by the plugins, data structures for all IO and models, and the macro for listener_fns.

## Adding host functions

Host functions needed to be added in contour_core as well as in this repo. To add a host function here, follow the patterns in the src/lib.rs file.
