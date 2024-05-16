use pyo3::{exceptions::PyTypeError, prelude::*};
use pdfium_render::prelude::*;
use std::{borrow::Cow, io::Cursor};


#[pyfunction]
fn rasterize_page(file_bytes: Vec<u8>, page_number: u16) -> PyResult<Cow<'static, [u8]>> {
    // Bind to the PDFium library
    let bindings = Pdfium::bind_to_system_library()
     .map_err(|e| PyErr::new::<PyTypeError, _>(format!("Failed to bind to PDFium library: {:?}", e)))?;

    let pdfium = Pdfium::new(bindings);

    // Load the PDF document from bytes
    let document = pdfium.load_pdf_from_byte_slice(&file_bytes, None)
        .map_err(|e| PyErr::new::<PyTypeError, _>(format!("Failed to load PDF: {:?}", e)))?;

    // Rendering configuration
    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    // Get the page and render it to an RGBA8 image
    let page = document.pages().get(page_number).unwrap();

    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);

    let _ = page.render_with_config(&render_config)
        .map_err(|e| PyErr::new::<PyTypeError, _>(format!("Render failed: {:?}", e)))?
        .as_image()
        .as_rgba8()
        .ok_or_else(|| PyErr::new::<PyTypeError, _>("Failed to convert image to RGBA8"))?
        .write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| PyErr::new::<PyTypeError, _>(format!("Failed to write image: {:?}", e)))?;

    // Convert the image to a Vec<u8> and return as Cow<u8>
    let image_bytes = Cow::Owned(buf);
    Ok(image_bytes)
}


/// A Python module implemented in Rust.
#[pymodule]
fn docprompt_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rasterize_page, m)?)?;
    Ok(())
}
