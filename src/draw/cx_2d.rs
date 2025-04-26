use std::ops::{Deref, DerefMut};
use crate::platform::Cx;
use crate::platform::math::Vec2;
use crate::platform::draw_list::DrawListId;
use crate::draw::turtle::{Turtle, Walk};
use crate::draw::layout::Layout;
use crate::draw::rect::Rect;

pub struct Cx2d<'a> {
    pub cx: &'a mut Cx,
    pub turtles: Vec<Turtle>,
    pub overlay_id: Option<DrawListId>,
}

impl<'a> Deref for Cx2d<'a> {
    type Target = Cx;
    
    fn deref(&self) -> &Self::Target {
        self.cx
    }
}

impl<'a> DerefMut for Cx2d<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cx
    }
}

impl<'a> Cx2d<'a> {
    pub fn new(cx: &'a mut Cx) -> Self {
        Self {
            cx,
            turtles: Vec::with_capacity(64),
            overlay_id: None,
        }
    }
    
    pub fn begin_turtle(&mut self, layout: Layout) -> &mut Turtle {
        let rect = Rect::zero();
        let turtle = Turtle::new(rect, layout);
        self.turtles.push(turtle);
        self.turtles.last_mut().unwrap()
    }
    
    pub fn begin_sized_turtle(&mut self, size: Vec2, layout: Layout) -> &mut Turtle {
        let rect = Rect::new(0.0, 0.0, size.x, size.y);
        let turtle = Turtle::new(rect, layout);
        self.turtles.push(turtle);
        self.turtles.last_mut().unwrap()
    }
    
    pub fn end_turtle(&mut self) -> Option<Turtle> {
        self.turtles.pop()
    }
    
    pub fn peek_turtle(&self) -> Option<&Turtle> {
        self.turtles.last()
    }
    
    pub fn peek_turtle_mut(&mut self) -> Option<&mut Turtle> {
        self.turtles.last_mut()
    }
    
    pub fn peek_walk_turtle(&self, walk: Walk) -> Rect {
        if let Some(turtle) = self.peek_turtle() {
            match walk {
                Walk::Size(size) => {
                    let mut rect = turtle.rect.clone();
                    rect.size = size;
                    rect
                },
                Walk::Compute => {
                    let content_size = turtle.compute_content_size();
                    let mut rect = turtle.rect.clone();
                    rect.size = content_size;
                    rect
                },
            }
        } else {
            Rect::zero()
        }
    }
    
    pub fn walk_turtle(&mut self, walk: Walk) -> Rect {
        if let Some(turtle) = self.peek_turtle_mut() {
            match walk {
                Walk::Size(size) => {
                    turtle.rect.size = size;
                    turtle.rect
                },
                Walk::Compute => {
                    let content_size = turtle.compute_content_size();
                    turtle.rect.size = content_size;
                    turtle.rect
                },
            }
        } else {
            Rect::zero()
        }
    }
    
    pub fn add_turtle_item(&mut self, size: Vec2) -> Option<Rect> {
        if let Some(turtle) = self.peek_turtle_mut() {
            Some(turtle.add_item(size, None))
        } else {
            None
        }
    }
}



