# tile-map-generator

Builds an image of a procedural generated map by stitching together sprite images from a specified spritesheet.

## How to use
1. Modify the `config.ron` file to indicate the x,y location of the wall/floor images on the spritesheet.
2. Run `cargo run <path-to-spritesheet>`
3. View the generated `output.png` file.

## TODOs
- [ ] Allow specification of more sprite images to draw.
- [ ] Better error handling.
- [ ] Prevent infinite looping during room generation.
- [ ] Add example of usage.
