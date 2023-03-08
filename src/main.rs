use pix_engine::prelude::*;
use image::{DynamicImage};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
	}

fn list_files(path_dir: &str, filetype: &str) -> Vec<String>{
    let mut files: Vec<String> = Vec::new();
    for element in std::path::Path::new(path_dir).read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == filetype {
                files.push(path.into_os_string().into_string().unwrap());
            	}
        	}
    	}
    files
	}

fn load_scale_jpg(filename: &str, new_width: u32) -> Result<(u32, u32, Vec<u8>, PixelFormat), String> {
	let img = match image::open(filename) {
		Ok(img) => img,
		Err(_) => return Err("couldn't open file".to_string()),
			};
	let scale_factor: f32 = round(new_width as f32 / img.width() as f32, 2);
	println!("Loading {}", filename);
	let res_width = (img.width() as f32 * scale_factor) as u32;
	let res_height = (img.height() as f32 * scale_factor) as u32;
	let scaled_img = img.resize(res_width, res_height,image::imageops::FilterType::Lanczos3);

	let bytes = match scaled_img {
		DynamicImage::ImageRgb8(img) => img.into_raw(),
		DynamicImage::ImageRgba8(img) => img.into_raw(),
		_ => return Err("couldn't load file".to_string()),
    	};

	Ok((res_width, res_height, bytes, PixelFormat::Rgb))
	}

struct PopeMemos {
	images: Vec<Image>,
	}

impl PopeMemos {
	fn new() -> PixResult<Self> {
		let mut images: Vec<Image> = Vec::new();
		for file in list_files("img/", "jpg") {
    		match load_scale_jpg(file.as_str(), 128) {
        		Ok((width, height, data, format)) => {
					images.push(Image::from_vec(width, height, data, format));
        			},
				Err(err) => {
					eprintln!("Error loading {}: {}", file, err);
					continue;
					},
				}
			}
		Ok(Self { images })
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
			s.image(&self.images[i], [i as i32 *256 + 128, s.height()? as i32 / 2])?;
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
	
