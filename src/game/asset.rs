pub enum AssetType {
    Environment,
    Character,
    Object,
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
    pub fn get_width(&self) -> f64 {
        self.width
    }
    pub fn get_height(&self) -> f64 {
        self.height
    }
}