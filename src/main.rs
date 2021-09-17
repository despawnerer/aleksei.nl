use warp::filters;
use warp::http::header::{SET_COOKIE, CONTENT_TYPE, CACHE_CONTROL};
use warp::Filter;

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const COOKIE_NAME: &str = "one";
const INITIAL_FONT_SIZE: f32 = 16.0;
const MAX_FONT_SIZE: f32 = 65536.0;

#[tokio::main]
async fn main() {
    let hello = warp::any().and(filters::cookie::optional(COOKIE_NAME)).map(
        |maybe_font_size: Option<f32>| {
            let font_size = maybe_font_size.unwrap_or(INITIAL_FONT_SIZE);
            let letter_spacing = - font_size / 20.0;

            let next_font_size = if font_size < MAX_FONT_SIZE {
                font_size as f32 * 1.2
            } else {
                INITIAL_FONT_SIZE
            };

            let html = format!(
                r#"
<meta http-equiv="refresh" content="1;https://voronov.nl">

<title>Voronov</title>

<style>
html, body {{
    padding: 0;
    margin: 0;
    width: 100%;
    height: 100%;
}}

body {{
    background: white;
    color: black;
    font-family: sans-serif;
    font-weight: bold;
    font-size: {}px;
    letter-spacing: {}px;
    white-space: nowrap;
    overflow: hidden;
}}

span {{
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
}}
</style>

<span>Voronov</span>
        "#,
                font_size, letter_spacing
            );

            warp::http::response::Builder::new()
                .header(CONTENT_TYPE, "text/html; charset=utf-8")
                .header(SET_COOKIE, format!("{}={}", COOKIE_NAME, next_font_size))
                .header(CACHE_CONTROL, "no-store")
                .status(200)
                .body(html)
                .unwrap()
        },
    );

    warp::serve(hello).run((IP, PORT)).await;
}
