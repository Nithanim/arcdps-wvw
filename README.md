Addon for arcdps that displays the current state of a wvw map using the guild wars 2 api.
Currently, it is in very early stages and has a bad UI and also hardcoded data.

## Building

The repo contains a crate `exe` that allows testing on linux without the game.

`cargo build --bin arcdps_wvw_exe`
`cargo run --bin arcdps_wvw_exe`

To build the dll for arcdps, you must install the windows target via

```sh
rustup target add x86_64-pc-windows-gnu
```

and you have to have the mingw64 cross-compiler installed.

You can then create the dll via
`cargo build --target x86_64-pc-windows-gnu`

Warning: for some dumb reason, `imgui-sys` needs `rustc-link-lib=stdc++`.
So the resulting dll depends on `libstdc++-6.dll` and `libgcc_s_seh-1.dll`.
Find and copy them from the mingw64 files and place them alongside the dll in the game dir.



