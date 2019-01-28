#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub enum AssetType {
    Environment,
    Character,
    Object,
    RuzzleObject,
}

pub struct Asset {
    asset_type: AssetType,
    x_offset: f64,
    y_offset: f64,
    width: f64,
    height: f64,
}

impl Asset {
    pub fn new(asset_type: AssetType, x_offset: f64, y_offset: f64, width: f64, height: f64) -> Asset {
        Asset {
            asset_type,
            x_offset,
            y_offset,
            width,
            height,
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
    pub fn set_x_offset(&mut self, new_x_offset: f64) {
        self.x_offset = new_x_offset;
    }
    pub fn get_y_offset(&self) -> f64 {
        self.y_offset
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