use seija_app::ecs::{system::{Commands, Res, Local, Query}, prelude::Entity};
use seija_input::{Input, event::MouseButton};
use seija_pbr::PBRCameraInfo;
use seija_render::camera::camera::{Perspective, Camera};
use seija_transform::Transform;
use seija_core::{math::{Vec3, Quat, Vec2, EulerRot}, window::AppWindow, time::Time};

pub fn add_pbr_camera(commands:&mut Commands,pos:Vec3,r:Quat,w:&AppWindow) {
    let mut camera_commands = commands.spawn();
    let mut t = Transform::default();
    t.local.position = pos;
    t.local.rotation = r;
    camera_commands.insert(t);

    let mut per = Perspective::default();
    per.aspect_ratio = w.width() as f32 / w.height() as f32;
    let camera = Camera::from_3d(per);
    camera_commands.insert(camera);
    camera_commands.insert(PBRCameraInfo::default());
}



enum ControllerState {
    None,
    HOrVMove,
    Rotation
}

pub struct CameraTranController {
    move_speed:f32,
    hv_move_speed:f32,
    state:ControllerState,
    last_position:Vec2
}

impl Default for CameraTranController {
    fn default() -> Self {
        CameraTranController { 
            move_speed:10f32,
            hv_move_speed:5f32,
            state:ControllerState::None,
            last_position:Vec2::ZERO
         }
    }
}

pub fn update_camera_trans_system(time:Res<Time>,
                              input:Res<Input>,
                              mut controller:Local<CameraTranController>,
                              mut cameras:Query<(Entity,&Camera,&mut Transform)>) {
    
    if let Some((_entity,_camera,mut trans)) = cameras.iter_mut().next() {
        match controller.state {
            ControllerState::None => {
                if let Some(delta) = input.get_mouse_wheel() {
                    let foward = trans.global().rotation * -Vec3::Z;
                    //后滑
                    if delta.y > 0f32 {
                         trans.local.position += foward * time.delta_seconds() * controller.move_speed;
                    } else {
                         trans.local.position -= foward * time.delta_seconds() * controller.move_speed;
                    }
                }
                if input.get_mouse_down(MouseButton::Right) {
                    controller.state = ControllerState::Rotation
                }
                if input.get_mouse_down(MouseButton::Middle) {
                    controller.state = ControllerState::HOrVMove;
                    controller.last_position = input.mouse_position;
                }
            },
            ControllerState::Rotation => {
                let delta = input.mouse_position - controller.last_position;
                if delta.x != 0f32 {
                    let x_dir = if delta.x > 0f32 { -1f32 } else { 1f32 }; 
                    let (euler_y,euler_x,euler_z) = trans.local.rotation.to_euler(EulerRot::YXZ);
                    trans.local.rotation = Quat::from_euler(EulerRot::YXZ,euler_y + (x_dir * 80f32 * time.delta_seconds()).to_radians(), euler_x, euler_z);
                }
                if delta.y != 0f32 {
                    let y_dir = if delta.y > 0f32 { -1f32 } else { 1f32 }; 
                    let (euler_y,euler_x,euler_z) = trans.local.rotation.to_euler(EulerRot::YXZ);
                    trans.local.rotation = Quat::from_euler(EulerRot::YXZ,euler_y, euler_x + (y_dir * 80f32 * time.delta_seconds()).to_radians(), euler_z);
                }
                if input.get_mouse_up(MouseButton::Right) {
                    controller.state = ControllerState::None
                }
                controller.last_position = input.mouse_position;
            },
            ControllerState::HOrVMove => {
                let delta = input.mouse_position - controller.last_position;
                if delta.x != 0f32 {
                    let right = trans.global().rotation * Vec3::X;
                    trans.local.position += -delta.x * controller.hv_move_speed * right * time.delta_seconds();
                }
                if delta.y != 0f32 {
                    let top = trans.global().rotation * Vec3::Y;
                    trans.local.position += delta.y * controller.hv_move_speed * top * time.delta_seconds();
                }
                if input.get_mouse_up(MouseButton::Middle) {
                    controller.state = ControllerState::None
                }
                controller.last_position = input.mouse_position;
            }
        }
    }
}