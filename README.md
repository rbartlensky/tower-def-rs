# tower-def

## Introduction

This is my attempt at creating a tower defence game in Rust using the Amethyst
game engine.

### Implemented so far

* Runners choose a random path towards the castle
* Towers shoot on sight and deal damage (if a runner is hit twice, it dies)
* If you click on the X sprites, you create a tower (a default one for now)

### To be implemented

* Menus
* Level selector
* Tower selector
* Tower upgrades + effects (buffs, debuffs)
* Smart runners (runners that do interesting stuff)

## Images
![](https://raw.githubusercontent.com/rbartlensky/tower-def/master/td1.png)
![](https://raw.githubusercontent.com/rbartlensky/tower-def/master/td2.png)

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```


## Credits

* To Andre Mari Coppola for the awesome sprite pack! (check assets folder more
information about the artist)


## Resources

* Using tile maps in Amethyst: https://github.com/Temeez/Tiled-Amethyst-Example
* The Amethyst [book](https://book.amethyst.rs/stable/intro.html) and
[docs](https://docs.amethyst.rs/stable/amethyst/).
