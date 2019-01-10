use super::{Character, Direction};
use crate::game::{Asset, AssetType, Coordinate};
use crate::game::constants::{
    PLAYER_BASE_X_OFFSET,
    PLAYER_BASE_Y_OFFSET,
    PLAYER_WIDTH,
    PLAYER_HEIGHT
};

pub struct Player {
    asset: Asset,
    coordinate: Coordinate,
    direction: Direction,
}

impl Character for Player {
    fn get_asset(&self) -> &Asset {
        &self.asset
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Down => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 0f64),
            Direction::Right => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 2f64),
            Direction::Up => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 4f64),
            Direction::Left => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 6f64),
        }
    }
}

impl Player {
    pub fn new(coordinate: Coordinate, direction: Direction) -> Player {
        let asset = Asset::new(
            AssetType::Character,
            PLAYER_BASE_X_OFFSET,
            PLAYER_BASE_Y_OFFSET,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        );
        Player {
            asset,
            coordinate,
            direction,
        }
    }
}