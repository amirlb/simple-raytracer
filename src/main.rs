use std::io;
mod graphics;
mod geometry;
mod camera;
mod hittable;
use hittable::Sphere;
mod tracing;

fn sky_color(direction: geometry::Vec3) -> graphics::Color {
    let unit_direction = direction.normalize();
    let a = 0.5 * (unit_direction.1 + 1.0);
    graphics::Color {
        red: 1.0 - 0.5 * a,
        green: 1.0 - 0.3 * a,
        blue: 1.0,
    }
}

fn main() -> io::Result<()> {
    let mut scene = tracing::Scene::new();
    scene.sky = Box::new(&sky_color);
    scene.add_object(Sphere{ center: geometry::Vec3(0.0, 0.0, 1.0), radius: 0.5 });
    scene.add_object(Sphere{ center: geometry::Vec3(0.0, -101.0, 1.0), radius: 100.0 });

    let tracer = tracing::Tracer {
        image_width: 500,
        aspect_ratio: 16.0 / 9.0,
        samples_per_pixel: 10,
    };
    let image = tracer.render(&scene);

    image.save("pic.bmp")?;

    Ok(())
}
