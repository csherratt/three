extern crate three;

fn main() {
    let shaders = concat!(env!("CARGO_MANIFEST_DIR"), "/data/shaders");
    let mut win = three::Window::builder("Three-rs glTF example", &shaders).build();
    let mut light = win.factory.directional_light(0xFFFFFF, 7.0);
    light.look_at([1.0, 1.0, 1.0], [0.0, 0.0, 0.0], None);
    win.scene.add(&light);
    win.scene.background = three::Background::Color(0xC6F0FF);

    let default = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/Lantern.gltf");
    let path = std::env::args().nth(1).unwrap_or(default.into());
    let (group, mut cameras, _meshes) = win.factory.load_gltf(&path);
    win.scene.add(&group);

    let mut cam = if cameras.len() > 0 {
        cameras.swap_remove(0)
    } else {
        let default = win.factory.perspective_camera(60.0, 0.001 .. 100.0);
        win.scene.add(&default);
        default
    };

    let init = cam.sync(&win.scene).world_transform;
    let mut controls = three::controls::FirstPerson::builder(&cam)
        .position(init.position)
        .move_speed(4.0)
        .build();
    while win.update() && !three::KEY_ESCAPE.is_hit(&win.input) {
        controls.update(&win.input);
        win.render(&cam);
    }
}
