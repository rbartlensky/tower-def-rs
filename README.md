# tower-def

## Introduction

This is my attempt at creating a tower defense game in Rust using the Amethyst
game engine.

### Implemented so far

* Runners choose a random path towards the castle
* Towers shoot on sight and deal damage (if a runner is hit twice, it dies)
* Simple tower selector when you click on "X" tiles.
* Debuffs (when frost tower hits enemy it is slowed down)
* Tower upgrades
* Basic level selector

### To be implemented

* Menus
* More towers
* Smart runners (runners that do interesting stuff)

### To improve

* Make missles move at a constant speed towards the target. (My calculations
are wrong when I decide how much to append to the missle's transform)
* Tower selector (very basic atm, would like to add some sort-of border around
items in the selector)
* Debuffs (maybe implementing a trait is better than having closures?)
* Level selector


## Images
![](https://raw.githubusercontent.com/rbartlensky/tower-def-rs/master/td1.png)
![](https://raw.githubusercontent.com/rbartlensky/tower-def-rs/master/td2.png)

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
