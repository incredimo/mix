use crate::platform::math::Vec2;
use crate::draw::layout::{Layout, LayoutDirection, LayoutAlign};
use crate::draw::rect::Rect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Walk {
    Size(Vec2),
    Compute,
}

impl Walk {
    pub fn size(&self) -> Option<Vec2> {
        match self {
            Walk::Size(size) => Some(*size),
            Walk::Compute => None,
        }
    }

    pub fn width(&self) -> Option<f32> {
        self.size().map(|s| s.x)
    }

    pub fn height(&self) -> Option<f32> {
        self.size().map(|s| s.y)
    }
}

#[derive(Clone, Debug)]
pub struct Turtle {
    pub rect: Rect,
    pub layout: Layout,
    pub items: Vec<TurtleItem>,
}

#[derive(Clone, Debug)]
pub struct TurtleItem {
    pub rect: Rect,
    pub align_self: Option<LayoutAlign>,
}

impl Turtle {
    pub fn new(rect: Rect, layout: Layout) -> Self {
        Self {
            rect,
            layout,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, size: Vec2, align_self: Option<LayoutAlign>) -> Rect {
        let rect: Rect;

        match self.layout.direction {
            LayoutDirection::Horizontal => {
                let x = if self.items.is_empty() {
                    self.rect.pos.x + self.layout.padding.x
                } else {
                    let last_item = self.items.last().unwrap();
                    last_item.rect.right() + self.layout.spacing
                };

                let align = align_self.unwrap_or(self.layout.align_items);
                let y = self.rect.pos.y + self.layout.padding.y + align.resolve(
                    self.rect.height() - self.layout.padding.y * 2.0,
                    size.y
                );

                rect = Rect::new(x, y, size.x, size.y);
            },
            LayoutDirection::Vertical => {
                let y = if self.items.is_empty() {
                    self.rect.pos.y + self.layout.padding.y
                } else {
                    let last_item = self.items.last().unwrap();
                    last_item.rect.bottom() + self.layout.spacing
                };

                let align = align_self.unwrap_or(self.layout.align_items);
                let x = self.rect.pos.x + self.layout.padding.x + align.resolve(
                    self.rect.width() - self.layout.padding.x * 2.0,
                    size.x
                );

                rect = Rect::new(x, y, size.x, size.y);
            },
        }

        self.items.push(TurtleItem {
            rect,
            align_self,
        });

        rect
    }

    pub fn compute_content_size(&self) -> Vec2 {
        if self.items.is_empty() {
            return Vec2::zero();
        }

        match self.layout.direction {
            LayoutDirection::Horizontal => {
                let mut width: f32 = 0.0;
                let mut height: f32 = 0.0;

                for (i, item) in self.items.iter().enumerate() {
                    width += item.rect.width();
                    if i < self.items.len() - 1 {
                        width += self.layout.spacing;
                    }
                    height = height.max(item.rect.height());
                }

                Vec2::new(
                    width + self.layout.padding.x * 2.0,
                    height + self.layout.padding.y * 2.0,
                )
            },
            LayoutDirection::Vertical => {
                let mut width: f32 = 0.0;
                let mut height: f32 = 0.0;

                for (i, item) in self.items.iter().enumerate() {
                    height += item.rect.height();
                    if i < self.items.len() - 1 {
                        height += self.layout.spacing;
                    }
                    width = width.max(item.rect.width());
                }

                Vec2::new(
                    width + self.layout.padding.x * 2.0,
                    height + self.layout.padding.y * 2.0,
                )
            },
        }
    }
}



