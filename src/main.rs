use image::GrayImage;

struct Query {
	color: u8,  // 0-255 black: 0, white: 255
	diagonal: (i32, i32),  // (y, x)
	left_top: (i32, i32),  // (y, x)
}

impl Query {
	fn query(color: u8, height: i32, width: i32) -> QueryBuilder {
		QueryBuilder {
			color,
			diagonal: (height, width),
		}
	}
}

struct QueryBuilder {
	color: u8,
	diagonal: (i32, i32),
}

impl QueryBuilder {
	fn left_top(self, y: i32, x: i32) -> Query {
		Query {
			color: self.color,
			diagonal: self.diagonal,
			left_top: (y, x),
		}
	}

	fn left_bottom(self, y: i32, x: i32) -> Query {
		Query {
			color: self.color,
			diagonal: self.diagonal,
			left_top: (y - self.diagonal.0, x),
		}
	}

	fn right_top(self, y: i32, x: i32) -> Query {
		Query {
			color: self.color,
			diagonal: self.diagonal,
			left_top: (y, x - self.diagonal.1),
		}
	}

	fn right_bottom(self, y: i32, x: i32) -> Query {
		Query {
			color: self.color,
			diagonal: self.diagonal,
			left_top: (y - self.diagonal.0, x - self.diagonal.1),
		}
	}
}

fn draw(queries: &[Query], image_size: (u32, u32), pixel_length: u32) -> Result<GrayImage, &str> {
	fn div<T: num::Integer + Copy>(lhs: T, rhs: T) -> Result<T, &'static str> {
		if rhs == T::zero() {
			Err("Divide by zero")
		} else if lhs % rhs != T::zero() {
			Err("Not divisible")
		} else {
			Ok(lhs / rhs)
		}
	}

	let image_size = (div(image_size.0, pixel_length)?, div(image_size.1, pixel_length)?);

	let mut image = vec![0u8; (image_size.1 * image_size.0) as usize];

	for query in queries {
		let &Query{color, diagonal, left_top} = query;
		let diagonal = (div(diagonal.0, pixel_length as i32)?, div(diagonal.1, pixel_length as i32)?);
		let left_top = (div(left_top.0, pixel_length as i32)?, div(left_top.1, pixel_length as i32)?);

		for y in std::cmp::max(left_top.0, 0) as u32 .. std::cmp::min(left_top.0 + diagonal.0, image_size.0 as i32) as u32 {
			for x in std::cmp::max(left_top.1, 0) as u32 .. std::cmp::min(left_top.1 + diagonal.1, image_size.1 as i32) as u32 {
				image[(y * image_size.1 + x) as usize] = color;
			}
		}
	}

	GrayImage::from_raw(image_size.1, image_size.0, image).ok_or_else(|| "Invalid image size")
}

fn main() {
	let args: Vec<String> = std::env::args().collect();

	let output_dir = if args.len() > 2 {
		eprintln!("Usage: {} [<output_directory>]", args[0]);
		std::process::exit(1);
	} else if args.len() == 2 {
		&args[1]
	} else {
		"."
	};

	let query = |color, y, x| Query::query(color, y, x);
	let black = 0;
	let white = 255;

	let pixel_length = 25;

	let fence = 50;
	let area = (4000, 5875);
	let grain_storage = (250, 3250);
	let slope_half = (500, 1000);
	let water_zone = (1000, 3800);

	let area1 = vec! [
		query(white, area.0, area.1).left_top(fence, fence),
		query(black, grain_storage.0, grain_storage.1).left_bottom(fence + area.0, fence + 625),
		query(black, slope_half.0, slope_half.1).right_top(fence, fence + area.1),
	];

	let area2 = vec! [
		query(white, area.0, area.1).left_top(fence, fence),
		query(black, water_zone.0, water_zone.1).left_top(fence, fence + 1075 + slope_half.1),
		query(black, slope_half.0, slope_half.1).left_top(fence, fence + 1075),
	];

	let area3_yellow = vec! [
		query(white, area.0, area.1).left_top(fence, fence),
	];

	let area3_storage = vec! [
		query(white, area.0, 500 + 2250).left_top(fence, fence),
	];

	draw(&area1, ((area.0 + fence * 2) as u32, (area.1 + fence * 2) as u32), pixel_length).unwrap().save(output_dir.clone().push_str("area1.png")).unwrap();
	draw(&area2, ((area.0 + fence * 1) as u32, (area.1 + fence * 2) as u32), pixel_length).unwrap().save(output_dir.clone().push_str("area2.png")).unwrap();
	draw(&area3_yellow, ((area.0 + fence * 1) as u32, (area.1 + fence * 2) as u32), pixel_length).unwrap().save(output_dir.clone().push_str("area3_yellow.png")).unwrap();
	draw(&area3_storage, ((area.0 + fence * 1) as u32, (area.1 + fence * 2) as u32), pixel_length).unwrap().save(output_dir.clone().push_str("area3_storage.png")).unwrap();
}