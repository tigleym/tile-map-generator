use std::env;
use std::path::Path;
use std::{fs::File};

use image::{ImageBuffer, GenericImageView};
use ron::de::{from_reader};
use serde::Deserialize;

mod dungeon_generator;
use crate::dungeon_generator::{create_map, RoomConfig, Wall};

#[derive(Debug, Deserialize)]
struct Config {
    height: u32,
    width: u32,
    tile_size: u32,
    wall_tile_h: (u32, u32),
    wall_tile_v_right: (u32, u32),
    wall_tile_v_left: (u32, u32),
    floor_tile: (u32, u32),
    max_room_size: u32,
    min_room_size: u32,
    max_rooms: u32,
    min_rooms: u32,
}

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    // Read config file to build image.
    let config_file = File::open(&Path::new("config.ron")).expect("Failed opening config file");
    let config: Config = match from_reader(config_file) {
      Ok(c) => c,
      Err(e) => {
        println!("Failed to load config: {}", e);

        std::process::exit(1);
      }
    };

    let imgx = config.width;
    let imgy = config.height;
    let tile_size = config.tile_size;
    let wall_tile_h_x = config.wall_tile_h.0;
    let wall_tile_h_y = config.wall_tile_h.1;
    let wall_tile_vr_x = config.wall_tile_v_right.0;
    let wall_tile_vr_y = config.wall_tile_v_right.1;
    let wall_tile_vl_x = config.wall_tile_v_left.0;
    let wall_tile_vl_y = config.wall_tile_v_left.1;
    let floor_tile_x = config.floor_tile.0;
    let floor_tile_y = config.floor_tile.1;

    let room_config = RoomConfig {
      max_room_size: config.max_room_size,
      min_room_size: config.min_room_size,
      max_rooms: config.max_rooms,
      min_rooms: config.min_rooms,
    };

    let tiles = create_map(&imgx, &imgy, &tile_size, &room_config);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = ImageBuffer::new(imgx, imgy);
    let texture = image::open(&Path::new(&file)).unwrap();

    // Draw the map
    for tile in tiles {
      let x_pixel_offset = tile.x * tile_size;
      let y_pixel_offset = tile.y * tile_size;

      for x in x_pixel_offset..(x_pixel_offset + tile_size) {
        for y in y_pixel_offset..(y_pixel_offset + tile_size) {

          let mut sprite_x = 0;
          let mut sprite_y = 0;

          // println!("sprite type: {:?}", tile.sprite_type);
          match tile.sprite_type {
            Some(Wall::Top) => {
              sprite_x = wall_tile_h_x;
              sprite_y = wall_tile_h_y;
            },
            Some(Wall::Right) => {
              sprite_x = wall_tile_vr_x;
              sprite_y = wall_tile_vr_y;
            },
            Some(Wall::Bottom) => {
              sprite_x = wall_tile_h_x;
              sprite_y = wall_tile_h_y;
            },
            Some(Wall::Left) => {
              sprite_x = wall_tile_vl_x;
              sprite_y = wall_tile_vl_y;
            },
            Some(Wall::Floor) => {
              sprite_x = floor_tile_x;
              sprite_y = floor_tile_y;
            }
            None => {},
          };

          let pix_x = sprite_x + (x % tile_size);
          let pix_y = sprite_y + (y % tile_size);

          let pixel = texture.get_pixel(pix_x as u32, pix_y as u32);
          imgbuf.put_pixel(x as u32, y as u32, pixel);
        }
      }
    }

    // Save the image as output.png
    imgbuf.save("output.png").unwrap()
}
