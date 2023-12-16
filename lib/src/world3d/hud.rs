use std::ops::Div;
use imgui_sys::{igGetColorU32Vec4, igGetForegroundDrawList, ImDrawList_PathLineTo, ImDrawList_PathStroke, ImVec2, ImVec4};
use mumblelink_reader::mumble_link::{MumbleLinkData, MumbleLinkDataReader};
use nalgebra::{Const, OMatrix, Point2, Point3, U4, Vector4};
use crate::mumble::GuildwarsContext;
use crate::settings::Settings;
use crate::world3d::helpers;
use crate::world3d::screen::get_screen_size;

pub fn render_hud(settings: &Settings, ml: &MumbleLinkData) {
    if get_current_map_id(ml) != 0 {
        render_hud_(settings, ml);
    }
}
pub fn render_hud_(settings: &Settings, ml: &MumbleLinkData) {
    let screen_size = get_screen_size();
    let view_projection = helpers::get_view_projection_matrix(&ml);

    if settings.show_objectives_overlay {
        let gu = GraphicsUniform {
            screen_size,
            view_projection,
        };

        let target = Point3::new(-62.843487, 23.980408, 229.41426);
        gu.render(target, |imgui_coords| unsafe {
            let draw_list = igGetForegroundDrawList();
            ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x - 100.0, imgui_coords.y - 100.0));
            ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x + 100.0, imgui_coords.y - 100.0));
            ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x + 100.0, imgui_coords.y + 100.0));
            ImDrawList_PathLineTo(draw_list, ImVec2::new(imgui_coords.x - 100.0, imgui_coords.y + 100.0));
            ImDrawList_PathStroke(draw_list, igGetColorU32Vec4(ImVec4::new(1.0, 0.0, 0.0, 1.0)), true, 6.0);
        })
    }
}

fn get_current_map_id(ml: &MumbleLinkData) -> u32 {
    let gw2context = ml.read_context_into_struct::<GuildwarsContext>();
    return gw2context.map_id;
}

struct GraphicsUniform {
    screen_size: ImVec2,
    view_projection: OMatrix<f32, U4, Const<4>>,
}

impl GraphicsUniform {
    fn render<F>(&self, world_coords: Point3<f32>, render_fn: F)
        where
            F: FnOnce(Point2<f32>) {
        let clip_space_coords: Vector4<f32> = self.view_projection * world_coords.to_homogeneous();
        if clip_space_coords.z > 0.0 {
            let normalized_device_coordinates = clip_space_coords.div(clip_space_coords.w);
            let normalized_device_coordinates = Point2::new(normalized_device_coordinates.x * -1.0 /* quick fix for inverted x, fix root cause! */, normalized_device_coordinates.y);

            let screen_coords = Point2::new((normalized_device_coordinates.x + 1.0) / 2.0 * self.screen_size.x, (normalized_device_coordinates.y + 1.0) / 2.0 * self.screen_size.y);
            let imgui_coords = Point2::new(screen_coords.x, self.screen_size.y - screen_coords.y);

            render_fn(imgui_coords);
        }
    }
}
