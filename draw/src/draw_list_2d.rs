use mix_platform::draw_list::DrawListId;
use mix_platform::math::Mat4;
use mix_platform::Cx;
use crate::rect::Rect;
use crate::cx_2d::Cx2d;

#[derive(Clone, Debug)]
pub struct DrawList2d {
    pub draw_list_id: DrawListId,
    pub dirty_check_rect: Rect,
}

impl DrawList2d {
    pub fn new(cx: &mut Cx) -> Self {
        let draw_list_id = cx.create_draw_list();

        Self {
            draw_list_id,
            dirty_check_rect: Rect::zero(),
        }
    }

    pub fn id(&self) -> DrawListId {
        self.draw_list_id
    }

    pub fn begin(&mut self, cx: &mut Cx2d) -> bool {
        if let Some(draw_list) = cx.draw_lists.get_mut(&self.draw_list_id) {
            draw_list.clear();
            true
        } else {
            false
        }
    }

    pub fn begin_always(&mut self, cx: &mut Cx2d) {
        if let Some(draw_list) = cx.draw_lists.get_mut(&self.draw_list_id) {
            draw_list.clear();
        }
    }

    pub fn end(&mut self, _cx: &mut Cx2d) {
        // Finalize the draw list
    }

    pub fn set_view_transform(&mut self, cx: &mut Cx2d, transform: &Mat4) {
        if let Some(draw_list) = cx.draw_lists.get_mut(&self.draw_list_id) {
            draw_list.set_view_transform(*transform);
        }
    }
}
