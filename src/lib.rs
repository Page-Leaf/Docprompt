use std::borrow::Cow;
use std::io::Cursor;
use std::fmt;
use pyo3::prelude::*;
use pyo3::create_exception;
use image::{GenericImageView, ImageFormat};

create_exception!(mymodule, ValueError, pyo3::exceptions::PyException);

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResizeMode {
    Thumbnail,
    Resize,
}

impl fmt::Display for ResizeMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResizeMode::Thumbnail => write!(f, "thumbnail"),
            ResizeMode::Resize => write!(f, "resize"),
        }
    }
}

impl pyo3::FromPyObject<'_> for ResizeMode {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        let s = ob.str()?;
        match s.to_str()? {
            "thumbnail" => Ok(ResizeMode::Thumbnail),
            "resize" => Ok(ResizeMode::Resize),
            _ => Err(ValueError::new_err("Invalid ResizeMode")),
        }
    }
}



#[pyfunction]
fn resize_image_to_file_size_limit(
    image_bytes: &[u8],
    max_file_size_bytes: usize,
    resize_mode: Option<ResizeMode>,
    resize_step_size: Option<f64>,
) -> PyResult<Cow<[u8]>> {
    let resize_mode = resize_mode.unwrap_or(ResizeMode::Thumbnail);
    let resize_step_size = resize_step_size.unwrap_or(0.1);

    if resize_step_size <= 0.0 || resize_step_size >= 1.0 {
        return Err(ValueError::new_err("resize_step_size must be between 0 and 1"));
    }

    if image_bytes.len() <= max_file_size_bytes {
        return Ok(image_bytes.to_vec().into());
    }

    let mut output_bytes = image_bytes.to_vec();
    let mut scale_factor = 1.0;

    while output_bytes.len() > max_file_size_bytes {
        let image = image::load_from_memory(&output_bytes)
            .map_err(|e| ValueError::new_err(e.to_string()))?;
        let (width, height) = image.dimensions();

        scale_factor *= 1.0 - resize_step_size;
        let new_width = (width as f64 * scale_factor) as u32;
        let new_height = (height as f64 * scale_factor) as u32;

        if new_width <= 200 || new_height <= 200 {
            eprintln!("Image could not be resized to under {} bytes. Reached {} bytes at {}x{} dimensions.",
                      max_file_size_bytes, output_bytes.len(), new_width, new_height);
            break;
        }

        let resized_image = match resize_mode {
            ResizeMode::Thumbnail => image.thumbnail(new_width, new_height),
            ResizeMode::Resize => image.resize(new_width, new_height, image::imageops::FilterType::Triangle),
        };

        output_bytes.clear();
        resized_image.write_to(&mut Cursor::new(&mut output_bytes), ImageFormat::Png)
            .map_err(|e| ValueError::new_err(e.to_string()))?;
    }

    Ok(output_bytes.into())
}

#[pymodule]
fn docprompt_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(resize_image_to_file_size_limit, m)?)?;
    // m.add_function(wrap_pyfunction!(resize_pil_image, m)?)?;
    // m.add_function(wrap_pyfunction!(process_raster_image, m)?)?;
    Ok(())
}