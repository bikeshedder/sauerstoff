use bevy::math::{Mat3, Mat4, Quat, Vec3};
use bresenham::Bresenham;
use image::{DynamicImage, GrayImage, ImageBuffer};

pub struct Map {
    collision_map: GrayImage,
}

impl Default for Map {
    fn default() -> Self {
        // FIXME this image loading is kinda inefficient
        let reader = image::io::Reader::open("assets/map/map-collision.png").unwrap();
        let img = reader.decode().unwrap();
        let img = img.into_luma8();
        Self { collision_map: img }
    }
}

impl Map {
    /// Find the far most non-colliding position on the map for a given
    /// target coordinate coming from a given source coordinate.
    pub fn collide(&self, source: Vec3, target: Vec3) -> Option<Vec3> {
        let img_width: u32 = self.collision_map.width();
        let img_height: u32 = self.collision_map.height();
        let mat = Mat4::from_scale_rotation_translation(
            Vec3::new(1.0, -1.0, 1.0),
            Quat::IDENTITY,
            Vec3::new((img_width as f32) / 2.0, (img_height as f32) / 2.0, 0.0),
        );
        let img_target = mat.transform_point3(target);
        let img_target = (img_target.x as isize, img_target.y as isize);
        let img_source = mat.transform_point3(source);
        let img_source = (img_source.x as isize, img_source.y as isize);
        // FIXME make sure x and y aren't completely out of bounds
        let z = Bresenham::new(img_target, img_source).collect::<Vec<_>>();
        let last_non_colliding = Bresenham::new(img_source, img_target)
            .take_while(|(x, y)| self.collision_map.get_pixel(*x as u32, *y as u32).0[0] > 0)
            .last();
        // We should probably return an error if the player managed to wander in
        // blocking territory.
        last_non_colliding.map(|p| {
            mat.inverse()
                .transform_point3(Vec3::new(p.0 as f32, p.1 as f32, 0.0))
        })
    }
}
