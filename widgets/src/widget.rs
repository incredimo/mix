use mix_platform::Cx;
use mix_platform::event::Event;
use mix_draw::Cx2d;
use std::any::Any;

pub trait Widget: Any {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event);
    fn draw(&mut self, cx: &mut Cx2d) -> DrawStep;
}

pub enum DrawStep {
    Done,
    Redraw,
}

impl DrawStep {
    pub fn done() -> Self {
        DrawStep::Done
    }

    pub fn redraw() -> Self {
        DrawStep::Redraw
    }

    pub fn is_redraw(&self) -> bool {
        match self {
            DrawStep::Redraw => true,
            _ => false,
        }
    }
}

pub struct WidgetRef {
    pub widget: Box<dyn Widget>,
}

impl WidgetRef {
    pub fn new<W: Widget + 'static>(widget: W) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.widget.handle_event(cx, event);
    }

    pub fn draw(&mut self, cx: &mut Cx2d) -> DrawStep {
        self.widget.draw(cx)
    }

    pub fn downcast_ref<T: Widget + WidgetExt + 'static>(&self) -> Option<&T> {
        (self.widget.as_ref() as &dyn Any).downcast_ref::<T>()
    }

    pub fn downcast_mut<T: Widget + WidgetExt + 'static>(&mut self) -> Option<&mut T> {
        (self.widget.as_mut() as &mut dyn Any).downcast_mut::<T>()
    }
}

pub trait WidgetExt: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + 'static> WidgetExt for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
