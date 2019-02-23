use crate::game::Direction;

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub enum AssetType {
    RuzzleObject,
    RuzzleUI,
    RuzzleEnvironment,
    RuzzleCharacters,
    RuzzleBackground,
}

pub struct Asset {
    asset_type: AssetType,
    x_offset: f64,
    y_offset: f64,
    width: f64,
    height: f64,
    sprite_x_offsets: Option<(f64, f64, f64, f64)>,
    sprite_y_offsets: Option<(f64, f64, f64, f64)>,
}

impl Asset {
    pub fn new(asset_type: AssetType, x_offset: f64, y_offset: f64, width: f64, height: f64, sprite_x_offsets: Option<(f64, f64, f64, f64)>, sprite_y_offsets: Option<(f64, f64, f64, f64)>) -> Asset {
        Asset {
            asset_type,
            x_offset,
            y_offset,
            width,
            height,
            sprite_x_offsets,
            sprite_y_offsets
        }
    }
    pub fn get_type(&self) -> &AssetType {
        &self.asset_type
    }
    pub fn set_asset_type(&mut self, new_asset_type: AssetType) {
        self.asset_type = new_asset_type;
    }
    pub fn get_x_offset(&self) -> f64 {
        self.x_offset
    }
    pub fn get_x_offset_by_direction(&self, direction: Direction) -> f64 {
        if let Some(sprite_x_offsets) = self.sprite_x_offsets {
            match direction {
                Direction::Up => sprite_x_offsets.0,
                Direction::Right => sprite_x_offsets.1,
                Direction::Down => sprite_x_offsets.2,
                Direction::Left => sprite_x_offsets.3,
            }
        } else {
            self.x_offset
        }
    }
    pub fn set_x_offset(&mut self, new_x_offset: f64) {
        self.x_offset = new_x_offset;
    }
    pub fn get_y_offset(&self) -> f64 {
        self.y_offset
    }
    pub fn get_y_offset_by_direction(&self, direction: Direction) -> f64 {
        if let Some(sprite_y_offsets) = self.sprite_y_offsets {
            match direction {
                Direction::Up => sprite_y_offsets.0,
                Direction::Right => sprite_y_offsets.1,
                Direction::Down => sprite_y_offsets.2,
                Direction::Left => sprite_y_offsets.3,
            }
        } else {
            self.y_offset
        }
    }
    pub fn set_y_offset(&mut self, new_y_offset: f64) {
        self.y_offset = new_y_offset;
    }
    pub fn set_width(&mut self, new_width: f64) {
        self.width = new_width;
    }
    pub fn set_height(&mut self, new_height: f64) {
        self.height = new_height;
    }
    pub fn get_width(&self) -> f64 {
        self.width
    }
    pub fn get_height(&self) -> f64 {
        self.height
    }
}