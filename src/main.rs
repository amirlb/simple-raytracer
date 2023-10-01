mod graphics;
mod geometry;
mod camera;
mod scene;
mod sphere;
mod material;

use geometry::Vec3;
use graphics::Color;
use material::Material;
use sphere::Sphere;

const SKY_BLUE: Color = Color{ red: 0.5, green: 0.7, blue: 1.0 };

fn sky_color(direction: Vec3) -> Color {
    let unit_direction = direction.normalize();
    let a = 0.5 * (unit_direction.1 + 1.0);
    Color::mix(graphics::WHITE, SKY_BLUE, a)
}

fn main() -> std::io::Result<()> {
    let mut scene = scene::Scene::new();
    scene.sky = Box::new(&sky_color);

    let material_ground = Material{albedo: Color{red: 0.8, green: 0.8, blue: 0.0}, polish: 0.0};
    let material_center = Material{albedo: Color{red: 0.7, green: 0.3, blue: 0.3}, polish: 0.0};
    let material_left   = Material{albedo: Color{red: 0.8, green: 0.8, blue: 0.8}, polish: 0.7};
    let material_right  = Material{albedo: Color{red: 0.8, green: 0.6, blue: 0.2}, polish: 0.1};

    scene.add_object(Sphere{ center: Vec3(0.0, -100.5, 1.0), radius: 100.0 }, material_ground);
    scene.add_object(Sphere{ center: Vec3(0.0, 0.0, 1.0), radius: 0.5 }, material_center);
    scene.add_object(Sphere{ center: Vec3(-1.0, 0.0, 1.0), radius: 0.5 }, material_left);
    scene.add_object(Sphere{ center: Vec3(1.0, 0.0, 1.0), radius: 0.5 }, material_right);

    let camera = camera::Camera::new(
        camera::Bearings {
            lookfrom: Vec3(0.0, 0.0, -3.0),
            lookat: Vec3(0.0, 0.0, 1.0),
            up: Vec3(0.0, 1.0, 0.0),
            fov_degrees: 28.1,
            // lookfrom: Vec3(-2.0, 2.0, -1.0),
            // lookat: Vec3(0.0, 0.0, 1.0),
            // up: Vec3(0.0, 1.0, 0.0),
            // fov_degrees: 20.0,
        },
        camera::ImageSettings {
            image_width: 600,
            aspect_ratio: 16.0 / 9.0,
        },
        camera::RenderSettings::shallow(),
    );
    let image = camera.render(scene);

    image.save("pic.bmp")?;

    Ok(())
}
