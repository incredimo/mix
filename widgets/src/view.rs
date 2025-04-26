use mix_platform::Cx;
use mix_platform::event::Event;
use mix_platform::area::Area;
use mix_draw::Cx2d;
use mix_draw::draw_list_2d::DrawList2d;
use mix_draw::layout::Layout;
use mix_draw::turtle::Walk;
use crate::widget::{Widget, DrawStep};

pub struct View {
    pub layout: Layout,
    pub draw_list: DrawList2d,
    pub area: Area,
    pub children: Vec<Box<dyn Widget>>,
}

impl View {
    pub fn new(cx: &mut Cx) -> Self {
        Self {
            layout: Layout::vertical(),
            draw_list: DrawList2d::new(cx),
            area: cx.create_area(),
            children: Vec::new(),
        }
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_horizontal_layout(mut self) -> Self {
        self.layout = Layout::horizontal();
        self
    }

    pub fn with_vertical_layout(mut self) -> Self {
        self.layout = Layout::vertical();
        self
    }

    pub fn add_child<W: Widget + 'static>(&mut self, child: W) {
        self.children.push(Box::new(child));
    }
}

impl Widget for View {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        for child in &mut self.children {
            child.handle_event(cx, event);
        }
    }

    fn draw(&mut self, cx: &mut Cx2d) -> DrawStep {
        self.draw_list.begin(cx);

        cx.begin_turtle(self.layout.clone());

        let mut needs_redraw = false;

        for child in &mut self.children {
            let draw_step = child.draw(cx);
            if draw_step.is_redraw() {
                needs_redraw = true;
            }
        }

        let _rect = cx.walk_turtle(Walk::Compute);

        cx.end_turtle();

        self.draw_list.end(cx);

        if needs_redraw {
            DrawStep::redraw()
        } else {
            DrawStep::done()
        }
    }
}
