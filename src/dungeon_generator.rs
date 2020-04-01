use std::cmp::{min, max};
use rand::Rng;

#[derive(Debug)]
pub enum Wall {
  Left,
  Right,
  Top,
  Bottom,
  Floor
}

pub struct Tile {
  pub x: u32,
  pub y: u32,
  pub empty: bool,
  pub sprite_type: Option<Wall>,
}

impl Tile {
  pub fn floor(&mut self) {
    self.empty = true;
  }

  pub fn wall(&mut self, wall: Wall) {
    self.sprite_type = Some(wall);
  }

  pub fn north(&self) -> u32 {
    self.y + 1
  }

  pub fn south(&self) -> u32 {
    self.y - 1
  }

  pub fn east(&self) -> u32 {
    self.x + 1
  }

  pub fn west(&self) -> u32 {
    self.x - 1
  }
}

struct Point {
  x: u32,
  y: u32,
}

#[derive(Clone, Copy)]
struct Rect {
  x: u32,
  y: u32,
  w: u32,
  h: u32,
}

impl Rect {
  pub fn intersects_with(&self, rect: &Rect) -> bool {
    (self.x <= (rect.x + rect.w)) &&
    ((self.x + self.w) >= rect.x) &&
    (self.y <= (rect.y + rect.h)) &&
    ((self.y + self.h) >= rect.y)
  }

  pub fn center(&self) -> Point {
    let center_x = (self.x + (self.x + self.w)) / 2;
    let center_y = (self.y + (self.y + self.h)) / 2;

    Point { x: center_x, y: center_y }
  }
}

pub struct RoomConfig {
  pub max_room_size: u32,
  pub min_room_size: u32,
  pub max_rooms: u32,
  pub min_rooms: u32,
}

pub fn create_map(width: &u32, height: &u32, tile_size: &u32, room_config: &RoomConfig) -> Vec<Tile> {
  let map_height = height / tile_size;
  let map_width = width / tile_size;
  let size = (map_height * map_width) as usize;
  let mut tiles = Vec::with_capacity(size);

  // Initialize map tiles.
  for x in 0..map_width {
    for y in 0..map_height {
      let tile = Tile {
        x: x as u32,
        y: y as u32,
        empty: false,
        sprite_type: None,
      };
      tiles.push(tile);
    }
  }

  // Generate rooms.
  let mut rng = rand::thread_rng();
  let num_rooms = rng.gen_range(room_config.min_rooms, room_config.max_rooms + 1);
  let mut generated_rooms:Vec<Rect> = Vec::with_capacity(num_rooms as usize);

  loop {
    // random width and height
    let w = rng.gen_range(room_config.min_room_size, room_config.max_room_size + 1);
    let h = rng.gen_range(room_config.min_room_size, room_config.max_room_size + 1);

    // random position without going out of the boundaries of the map
    let bounds_x = map_width - w - 3;
    let bounds_y = map_height - h - 3;
    let x = rng.gen_range(1, bounds_x);
    let y = rng.gen_range(1, bounds_y);

    // create the room and check if it intersects with already existing rooms. If it
    // doesn't, store it.
    let new_room = Rect { x, y, w, h };

    let current_rooms = generated_rooms.clone();
    let mut iter = current_rooms.into_iter();
    let intersecting_rect = iter.find(|room| new_room.intersects_with(&room));

    match intersecting_rect {
      None => {
        create_new_room(&mut tiles, &new_room, &map_width, &map_height);
        generated_rooms.push(new_room);
      },
      Some(_) => {},
    };

    if generated_rooms.len() == num_rooms as usize {
      break;
    }
  }

  // Create tunnels between rooms.
  for (index, room) in generated_rooms.iter().enumerate() {
    if index < generated_rooms.len() - 1 {
      let current_center = room.center();
      let prev_center = &generated_rooms[index + 1].center();

      if rand::random() {
        // draw a horizontal corridor first, then vertical
        create_h_tunnel(&mut tiles, &map_width, &prev_center.x, &current_center.x, &prev_center.y);
        create_v_tunnel(&mut tiles, &map_width, &prev_center.y, &current_center.y, &current_center.x);
      } else {
        // draw a vertical corridor first, then horizontal
        create_v_tunnel(&mut tiles, &map_width, &prev_center.y, &current_center.y, &prev_center.x);
        create_h_tunnel(&mut tiles, &map_width, &prev_center.x, &current_center.x, &current_center.y);
      }
    }
  }

  tiles
}

fn create_new_room<'a> (tiles: &'a mut Vec<Tile>, room: &Rect, map_width: &u32, map_height: &u32) -> &'a mut Vec<Tile> {
  // go through the tiles in the rectangle and make them passable
  for x in room.x..room.x + room.w {
    for y in room.y..room.y + room.h {
      let index = (x * map_width + y) as usize;
      tiles[index].floor();
      tiles[index].wall(Wall::Floor);

      if (room.x + room.w < *map_width) && room.x > 0 && room.y > 0  && (room.y + room.h < *map_height) {
        if tiles[index].north() == (room.y + room.h)  {
          tiles[index].wall(Wall::Top);
        } else if tiles[index].south() == (room.y - 1) {
          tiles[index].wall(Wall::Bottom);
        }

        if tiles[index].east() == (room.x + room.w) {
          tiles[index].wall(Wall::Right);
        } else if tiles[index].west() == (room.x - 1) {
          tiles[index].wall(Wall::Left);
        }
      }
    }
  }

  tiles
}

fn create_h_tunnel<'a> (tiles: &'a mut Vec<Tile>, map_width: &u32, x1: &u32, x2: &u32, y: &u32) -> &'a mut Vec<Tile> {
  let min_x: u32 = min(*x1, *x2);
  let max_x: u32 = max(*x1, *x2);

  for x in min_x..max_x {
    let index = (x * map_width + y) as usize;
    tiles[index].floor();
    tiles[index].wall(Wall::Floor);

    let north_tile_index = tiles[index].north();
    let south_tile_index = tiles[index].south();
    let east_tile_index = tiles[index].east();
    let west_tile_index = tiles[index].west();

    match &tiles[(east_tile_index * map_width + y) as usize].sprite_type {
      None => {
        if x == min_x {
          tiles[(east_tile_index * map_width + y) as usize].wall(Wall::Top);
        }
      },
      _=> {},
    }

    match &tiles[(west_tile_index * map_width + y) as usize].sprite_type {
      None => {
        if x == min_x {
          tiles[(west_tile_index * map_width + y) as usize].wall(Wall::Top);
        }
      },
      _=> {},
    }

    match &tiles[(x * map_width + north_tile_index) as usize].sprite_type {
      Some(Wall::Bottom) | Some(Wall::Right) | Some(Wall::Left) | None => {
        tiles[(x * map_width + north_tile_index) as usize].wall(Wall::Top);
      },
      _ => {},
    }

    match &tiles[(x * map_width + south_tile_index) as usize].sprite_type {
      Some(Wall::Bottom) | Some(Wall::Right) | Some(Wall::Left) | None => {
        tiles[(x * map_width + south_tile_index) as usize].wall(Wall::Bottom);
      },
      _ => {},
    }
  }

  tiles
}

fn create_v_tunnel<'a> (tiles: &'a mut Vec<Tile>, map_width: &u32, y1: &u32, y2: &u32, x: &u32) -> &'a mut Vec<Tile> {
  let min_y: u32 = min(*y1, *y2);
  let max_y: u32 = max(*y1, *y2);

  for y in min_y..max_y {
    let index = (x * map_width + y) as usize;

    tiles[index].floor();
    tiles[index].wall(Wall::Floor);

    let east_tile_index = tiles[index].east();
    let west_tile_index = tiles[index].west();

    match &tiles[(east_tile_index * map_width + y) as usize].sprite_type {
      Some(Wall::Right) | Some(Wall::Left) | None => {
        tiles[(east_tile_index * map_width + y) as usize].wall(Wall::Right);
      },
      _ => {},
    }

    match &tiles[(west_tile_index * map_width + y) as usize].sprite_type {
      Some(Wall::Right)| Some(Wall::Left) | None => {
        tiles[(west_tile_index * map_width + y) as usize].wall(Wall::Left);
      },
      _ => {},
    }
  }

  tiles
}
