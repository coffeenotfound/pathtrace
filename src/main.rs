use anyhow::Context;
use cgmath::{Deg, EuclideanSpace, Matrix4, perspective, Point3, Rad, SquareMatrix, Transform, vec2, vec3, Vector3};
use image::{ImageFormat, Rgb, RgbImage};

fn main() {
	let path = "render.png";
	let resolution = vec2(720, 480);
	let mut image = RgbImage::new(resolution.x, resolution.y);
	
	// Set up raytracer
	let mut raytracer = Raytracer::new();
	raytracer.camera_pos = Point3::origin();
	
	// Render the image
	raytracer.render(&mut image);
	
	// Save image to disk
	let result = image.save_with_format(path, ImageFormat::Png)
		.with_context(|| "Failed to write png output file");
	
	match result {
		Ok(..) => println!("Successfully rendered image"),
		Err(err) => eprintln!("Failed to render with error: {err}"),
	}
}

pub struct Raytracer {
	pub camera_pos: Point3<f32>,
}

impl Raytracer {
	pub fn new() -> Self {
		Self {
			camera_pos: Point3::origin(),
		}
	}
	
	pub fn render(&mut self, image: &mut RgbImage) {
		let resolution = vec2(image.width(), image.height());
		let aspect_ratio = resolution.x as f32 / resolution.y as f32;
		
		let focal_length: f32 = 1.0;
		let viewport_height: f32 = 2.0;
		let viewport_width: f32;
		
		let vfov: Rad<f32> = Deg(50.0).into();
		
		let proj_mat = perspective(
			Deg(45.0),
			aspect_ratio,
			1.0/16.0,
			1024.0,
		);
		let view_mat = Matrix4::look_to_rh(
			self.camera_pos,
			-Vector3::unit_z(),
			Vector3::unit_y(),
		);
		
		let view_proj_mat = proj_mat * view_mat;
		let inv_view_proj_mat = view_proj_mat.invert()
			.expect("Failed to invert view projection matrix!");
		
		for y in 0..resolution.y {
			for x in 0..resolution.x {
				let frag = vec3::<f32>(
					x as f32 / (resolution.x - 1) as f32,
					y as f32 / (resolution.y - 1) as f32,
					0.0,
				);
				
				let out_pixel = Rgb(frag.map(|c| (c.clamp(0.0, 1.0) * 255.0) as u8).into());
				image.put_pixel(x, y, out_pixel);
			}
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct Ray {
	pub origin: Point3<f32>,
	/// Note: May not be normalized!
	pub dir: Vector3<f32>,
}

impl Ray {
	pub fn at(&self, t: f32) -> Point3<f32> {
		self.origin + (self.dir * t)
	}
}
