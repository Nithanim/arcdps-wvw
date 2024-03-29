# WvW utilities for arcdps

Addon for arcdps that does random stuff for WvW.

Current features:

* Minimap of a WvW map (basic)
* Compass
* World-Overlay on objectives that show the buff timer

You can find [images on the wiki](https://github.com/Nithanim/arcdps-wvw/wiki).

The latest build can be found here: https://github.com/Nithanim/arcdps-wvw/raw/artifacts/arcdps_wvw.dll

## Building

The repo contains a crate `exe` that allows testing on linux without the game.

`cargo run --bin arcdps_wvw_exe`

To build the dll for arcdps on linux, you must install the windows target via

```sh
rustup target add x86_64-pc-windows-gnu
```

and you have to have the mingw64 cross-compiler installed.

You can then create the dll via
`cargo build  --package arcdps_wvw --target x86_64-pc-windows-gnu`

Warning: for some dumb reason, `imgui-sys` needs `rustc-link-lib=stdc++`.
So the resulting dll depends on `libstdc++-6.dll` and `libgcc_s_seh-1.dll`.
Find and copy them from the mingw64 files and place them alongside the dll in the game dir.

If you are on windows, you should be able to build the dll with windows tools.
You most likely need to either install gcc (mingw64 for windows) somehow or [use msvc](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup).

## Ideas/todo

* Step counter between fights
* Show upgrades, tier and buff on map
* More data for overlay HUD for objectives
* Visualize claim change on map
* Share and display position on HUD of other players
* Second-squad helper
  * One person in second squad gathers data
  * Commander in main squad has a similar overview like for their main squad
* Voice invite helper
  * Define invite text
  * Button to copy text
  * Button to automatically post
* Inc warning for commander by players
  * Simple button?
  * Share location with commander

