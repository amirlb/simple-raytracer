mod graphics;
mod geometry;
mod camera;
mod scene;
mod tracing;
mod sphere;
mod material;

use std::io;
use graphics::Color;
use sphere::Sphere;
use std::sync::Arc;

const SKY_BLUE: Color = Color{ red: 0.5, green: 0.7, blue: 1.0 };

fn sky_color(direction: geometry::Vec3) -> Color {
    let unit_direction = direction.normalize();
    let a = 0.5 * (unit_direction.1 + 1.0);
    Color::mix(graphics::WHITE, SKY_BLUE, a)
}

fn main() -> io::Result<()> {
    let mut scene = scene::Scene::new();
    scene.sky = Box::new(&sky_color);
    scene.camera = camera::Camera {
        position: geometry::Vec3(0.0, 0.0, -3.0),
        focal_length: 4.0,
    };

    let material_ground = material::Material{albedo: Color{red: 0.8, green: 0.8, blue: 0.0}, polish: 0.0};
    let material_center = material::Material{albedo: Color{red: 0.7, green: 0.3, blue: 0.3}, polish: 0.0};
    let material_left   = material::Material{albedo: Color{red: 0.8, green: 0.8, blue: 0.8}, polish: 0.7};
    let material_right  = material::Material{albedo: Color{red: 0.8, green: 0.6, blue: 0.2}, polish: 0.1};

    scene.add_object(Sphere{ center: geometry::Vec3(0.0, -100.5, 1.0), radius: 100.0 }, material_ground);
    scene.add_object(Sphere{ center: geometry::Vec3(0.0, 0.0, 1.0), radius: 0.5 }, material_center);
    scene.add_object(Sphere{ center: geometry::Vec3(-1.0, 0.0, 1.0), radius: 0.5 }, material_left);
    scene.add_object(Sphere{ center: geometry::Vec3(1.0, 0.0, 1.0), radius: 0.5 }, material_right);

    let tracer = tracing::Tracer {
        image_width: 600,
        aspect_ratio: 16.0 / 9.0,
        samples_per_pixel: 100,
    };
    let image = tracer.render(Arc::new(scene));

    image.save("pic.bmp")?;

    Ok(())
}
