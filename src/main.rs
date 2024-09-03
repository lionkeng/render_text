use reqwest;
use skia_safe::{Color, Data, EncodedImageFormat, Image, Paint, Rect};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        // Download the image
        let url = "https://anewgocdn.us/app/demo/images/siteplans/Waterside%20sp_Estates_53c41c602417fae6.jpg";
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        let img_bytes = response.bytes().await?;
        let data = Data::new_copy(&img_bytes); // safe
        let img = Image::from_encoded(data).unwrap();

        // Create a surface with the image dimensions
        let (width, height) = (img.width() as i32, img.height() as i32);
        let mut surface = skia_safe::surfaces::raster_n32_premul((width, height))
            .expect("Failed to create surface");
        let mut ctx = surface.direct_context();

        // Get the canvas and draw the image
        let canvas = surface.canvas();
        canvas.draw_image(&img, (0, 0), None);
        // Draw the square
        let square_size = 100.0;
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let square_rect = Rect::from_xywh(
            center_x - square_size / 2.0,
            center_y - square_size / 2.0,
            square_size,
            square_size,
        );

        let mut fill_paint = Paint::default();
        fill_paint.set_color(Color::WHITE);
        canvas.draw_rect(square_rect, &fill_paint);

        let mut stroke_paint = Paint::default();
        stroke_paint.set_color(Color::RED);
        stroke_paint.set_style(skia_safe::paint::Style::Stroke);
        stroke_paint.set_stroke_width(4.0);
        canvas.draw_rect(square_rect, &stroke_paint);

        // Draw the text "7" in the rectangle
        let mut text_paint = Paint::default();
        text_paint.set_color(Color::BLACK);
        text_paint.set_anti_alias(true);

        let typeface = skia_safe::Typeface::default();
        let mut font = skia_safe::Font::new(typeface.clone(), 60.0);
        font.set_typeface(typeface);
        font.set_size(60.0);
        let text = "7";
        let text_bounds = font.measure_str(text, Some(&text_paint));
        let text_x = center_x - text_bounds.1.width() / 2.0;
        let text_y = center_y + text_bounds.1.height() / 2.0;

        canvas.draw_str(text, (text_x, text_y), &font, &text_paint);

        // Save the result as a PNG file in the root directory
        let image = surface.image_snapshot();
        let img_data_encoded = image
            .encode(ctx.as_mut(), EncodedImageFormat::JPEG, None)
            .unwrap();
        let mut file = std::fs::File::create("output.jpeg").unwrap();
        file.write_all(&img_data_encoded).unwrap();
        Ok(())
    })
}
