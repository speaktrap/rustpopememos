use pix_engine::prelude::*;
use image::{DynamicImage};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
	}

fn list_files(path_dir: &str, filetype: &str) -> Vec<String>{
    let mut faxvec: Vec<String> = Vec::new();
    for element in std::path::Path::new(path_dir).read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == filetype {
                faxvec.push(path.into_os_string().into_string().unwrap());
            	}
        	}
    	}
    faxvec
	}

fn load_scale_jpg(filename: &str, new_width: u32) -> Option<(u32, u32, Vec<u8>, PixelFormat)> {
	let img = match image::open(filename) {
		Ok(img) => img,
		Err(_) => return None,
			};
	let scale_factor: f32 = round(new_width as f32 / img.width() as f32, 6);
	println!("{}", scale_factor);
	let res_width = (img.width() as f32 * scale_factor) as u32;
	let res_height = (img.height() as f32 * scale_factor) as u32;
	let scaled_img = img.resize(res_width, res_height,image::imageops::FilterType::Lanczos3);

	let bytes = match scaled_img {
		DynamicImage::ImageRgb8(img) => img.into_raw(),
		DynamicImage::ImageRgba8(img) => img.into_raw(),
		_ => return None,
    	};

	Some((res_width, res_height, bytes, PixelFormat::Rgb))
	}

struct PopeMemos {
	image: Vec<Image>,
	}

impl PopeMemos {
	fn new() -> PixResult<Self> {
		let img_path: &str = "img/papiesz-powazny-biznesmen-bardzo.jpg";
		let (width, height, data, format): (u32, u32, Vec<u8>, PixelFormat) = load_scale_jpg(img_path, 512).unwrap();
		let image1 = Image::from_vec(width, height, data, format);
		let image = vec![image1];
		Ok(Self { image })
		}
	}

impl PixEngine for PopeMemos {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
    	s.background(Color::GRAY);
        s.blend_mode(BlendMode::Blend);
        s.image_mode(ImageMode::Center);
        Ok(())
	}

	fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
		s.clear()?;
		for i in 0..4 {
			s.image(&self.image[0], [i*256, s.height()? as i32 / 2])?;
			}
		Ok(())
		}
	}

fn main() -> PixResult<()> {
	for i in list_files("./img/", "jpg") {
		println!("{}", i);
		}
	let mut engine = Engine::builder()
		.dimensions(WIDTH, HEIGHT)
		.title("Pope memos")
		.show_frame_rate()
		.build()?; 
	let mut app = PopeMemos::new()?;
	engine.run(&mut app)
	}
	
