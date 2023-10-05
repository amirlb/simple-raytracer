mod graphics;
mod geometry;
mod camera;
mod scene;
mod shapes;
mod material;

use geometry::Vec3;
use graphics::Color;
use shapes::Sphere;
use shapes::Medium;

const SKY_BLUE: Color = Color{ red: 0.5, green: 0.7, blue: 1.0 };

fn sky_color(direction: Vec3) -> Color {
    let a = 0.5 * (direction.1 + 1.0);
    Color::mix(graphics::WHITE, SKY_BLUE, a)
}

fn main() -> std::io::Result<()> {
    let mut scene = scene::Scene::new();
    scene.sky = Box::new(&sky_color);

    let material_ground = material::Opaque{albedo: Color{red: 0.8, green: 0.8, blue: 0.0}, polish: 0.0};
    let material_center = material::Opaque{albedo: Color{red: 0.1, green: 0.2, blue: 0.5}, polish: 0.0};
    let material_left   = material::Transparent{refraction_index: 1.5};
    let material_left2   = material::Transparent{refraction_index: 1.5};
    let material_right  = material::Opaque{albedo: Color{red: 0.8, green: 0.6, blue: 0.2}, polish: 0.9};

    scene.add_object(Sphere{ center: Vec3(0.0, -100.5, 1.0), radius: 100.0 }, material_ground);
    scene.add_object(Sphere{ center: Vec3(0.0, 0.0, 1.0), radius: 0.5 }, material_center);
    scene.add_object(Sphere{ center: Vec3(-1.0, 0.0, 1.0), radius: 0.5 }, material_left);
    scene.add_object(Sphere{ center: Vec3(-1.0, 0.0, 1.0), radius: -0.4 }, material_left2);
    scene.add_object(Sphere{ center: Vec3(1.0, 0.0, 1.0), radius: 0.5 }, material_right);
    // scene.add_object(Medium{ shape: Box::new(Sphere{ center: Vec3(-1.0, 0.0, 1.0), radius: 0.5 }), density: 5.0 }, material::Gas{albedo: graphics::Color{red:0.9,green:0.9,blue:0.9}, isotropy: 0.2});

    let camera = camera::Camera::new(
        camera::Bearings {
            lookfrom: Vec3(0.0, 0.0, -3.0),
            lookat: Vec3(0.0, 0.0, 1.0),
            up: Vec3(0.0, 1.0, 0.0),
            fov_degrees: 28.1,
            // lookfrom: Vec3(-2.0, 2.0, -1.0),
            // fov_degrees: 20.0,
        },
        camera::ImageSettings {
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,
        },
        camera::RenderSettings::shallow(),
    );
    let image = camera.render(&scene);

    image.save("pic.bmp")?;

    Ok(())
}
