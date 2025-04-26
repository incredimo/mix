use mix_platform::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutSize {
    Fixed(f32),
    Flex(f32),
    Fill,
    Fit,
}

impl LayoutSize {
    pub fn resolve(&self, available: f32, content: f32) -> f32 {
        match self {
            LayoutSize::Fixed(size) => *size,
            LayoutSize::Flex(factor) => available * factor,
            LayoutSize::Fill => available,
            LayoutSize::Fit => content,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutAlign {
    Start,
    Center,
    End,
    Stretch,
}

impl LayoutAlign {
    pub fn resolve(&self, container: f32, item: f32) -> f32 {
        match self {
            LayoutAlign::Start => 0.0,
            LayoutAlign::Center => (container - item) * 0.5,
            LayoutAlign::End => container - item,
            LayoutAlign::Stretch => 0.0, // Item will be stretched to fill container
        }
    }
}

#[derive(Clone, Debug)]
pub struct Layout {
    pub direction: LayoutDirection,
    pub width: LayoutSize,
    pub height: LayoutSize,
    pub align_items: LayoutAlign,
    pub justify_content: LayoutAlign,
    pub padding: Vec2,
    pub spacing: f32,
}

impl Layout {
    pub fn horizontal() -> Self {
        Self {
            direction: LayoutDirection::Horizontal,
            width: LayoutSize::Fill,
            height: LayoutSize::Fit,
            align_items: LayoutAlign::Start,
            justify_content: LayoutAlign::Start,
            padding: Vec2::new(0.0, 0.0),
            spacing: 0.0,
        }
    }
    
    pub fn vertical() -> Self {
        Self {
            direction: LayoutDirection::Vertical,
            width: LayoutSize::Fill,
            height: LayoutSize::Fit,
            align_items: LayoutAlign::Start,
            justify_content: LayoutAlign::Start,
            padding: Vec2::new(0.0, 0.0),
            spacing: 0.0,
        }
    }
    
    pub fn with_width(mut self, width: LayoutSize) -> Self {
        self.width = width;
        self
    }
    
    pub fn with_height(mut self, height: LayoutSize) -> Self {
        self.height = height;
        self
    }
    
    pub fn with_align_items(mut self, align_items: LayoutAlign) -> Self {
        self.align_items = align_items;
        self
    }
    
    pub fn with_justify_content(mut self, justify_content: LayoutAlign) -> Self {
        self.justify_content = justify_content;
        self
    }
    
    pub fn with_padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }
    
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}
