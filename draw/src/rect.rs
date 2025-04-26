use mix_platform::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }
    
    pub fn from_pos_size(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }
    
    pub fn zero() -> Self {
        Self {
            pos: Vec2::zero(),
            size: Vec2::zero(),
        }
    }
    
    pub fn width(&self) -> f32 {
        self.size.x
    }
    
    pub fn height(&self) -> f32 {
        self.size.y
    }
    
    pub fn x(&self) -> f32 {
        self.pos.x
    }
    
    pub fn y(&self) -> f32 {
        self.pos.y
    }
    
    pub fn right(&self) -> f32 {
        self.pos.x + self.size.x
    }
    
    pub fn bottom(&self) -> f32 {
        self.pos.y + self.size.y
    }
    
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.pos.x && point.x <= self.right() &&
        point.y >= self.pos.y && point.y <= self.bottom()
    }
    
    pub fn intersects(&self, other: &Rect) -> bool {
        self.pos.x < other.right() && self.right() > other.pos.x &&
        self.pos.y < other.bottom() && self.bottom() > other.pos.y
    }
    
    pub fn union(&self, other: &Rect) -> Rect {
        let x = self.pos.x.min(other.pos.x);
        let y = self.pos.y.min(other.pos.y);
        let right = self.right().max(other.right());
        let bottom = self.bottom().max(other.bottom());
        
        Rect::new(x, y, right - x, bottom - y)
    }
    
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let x = self.pos.x.max(other.pos.x);
        let y = self.pos.y.max(other.pos.y);
        let right = self.right().min(other.right());
        let bottom = self.bottom().min(other.bottom());
        
        if right > x && bottom > y {
            Some(Rect::new(x, y, right - x, bottom - y))
        } else {
            None
        }
    }
}
