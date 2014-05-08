// vi: ts=4 sw=4

use rsfml::graphics::{IntRect, RenderWindow};
use rsfml::graphics::rc::Sprite;
use rsfml::system::vector2::{ToVec, Vector2f, Vector2u};

static zero: f32 = 0.0f32;
static one: f32 = 1.0f32;

pub struct Ground {
	window_size: Vector2f,
	image_size: Vector2f,
	frequency: f32,
	phase: f32,
	sprite: ~Sprite,
}

impl Ground {
	pub fn new(window_size: Vector2u, mut sprite: ~Sprite, frequency: f32) -> Ground {
		let texture_ref = sprite.get_texture()
			.expect("Ground's Sprite didn't have a texture");
		let mut texture = texture_ref.deref().borrow_mut();
		let image_size: Vector2u = texture.get_size();

		let rect: IntRect = sprite.get_texture_rect();
		sprite.set_texture_rect(&IntRect {
			left: rect.left,
			top: rect.top,
			width: rect.width * 3,
			height: rect.height,
		});
		texture.set_repeated(true);

		Ground {
			window_size: window_size.to_vector2f(),
			image_size: image_size.to_vector2f(),
			frequency: frequency,
			phase: zero,
			sprite: sprite,
		}
	}

	pub fn update(&mut self, seconds: f32) {
		let frequency = self.frequency;
		let wavelength = one / frequency;

		self.phase += seconds;
		while self.phase >= wavelength {
			self.phase -= wavelength;
		}

		let window_size: Vector2f = self.window_size;
		let image_size: Vector2f = self.image_size;
		let at_bottom: f32 = window_size.y - image_size.y;

		let new_position: Vector2f = Vector2f {
			x: image_size.x * (zero - self.phase * frequency),
			y: at_bottom,
		};
		self.sprite.set_position(&new_position);
	}

	pub fn draw(&self, window: &mut RenderWindow) {
		window.draw(self.sprite);
	}
}
