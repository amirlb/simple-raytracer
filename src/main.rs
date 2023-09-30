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
    scene.add_object(Sphere{ center: geometry::Vec3(0.0, 0.0, 1.0), radius: 0.5 }, material::Material{});
    scene.add_object(Sphere{ center: geometry::Vec3(0.0, -101.0, 1.0), radius: 100.5 }, material::Material{});

    let tracer = tracing::Tracer {
        image_width: 600,
        aspect_ratio: 16.0 / 9.0,
        samples_per_pixel: 100,
    };
    let image = tracer.render(Arc::new(scene));

    image.save("pic.bmp")?;

    Ok(())
}
