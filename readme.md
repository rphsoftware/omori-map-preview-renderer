# Omori Map Preview Renderer

Helps fix the issue of backgrounds being empty in RPG Maker.

## Usage

- Drop the exe in your `www_playtest` folder
- Double click it
- The files you want to use as background parallaxes will be located in `scaled/` (a folder that was just made)
- Now, when editing a map, if you need it previewed, drag the image from `scaled/` where the ID corresponds to the id of your map into `img/parallaxes` and set the image as a background parallax for the map.

## Compiling

`cargo build --release`