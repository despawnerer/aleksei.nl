use warp::filters;
use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH, SET_COOKIE};
use warp::Filter;

const REFRESH_TARGET: &str = "2;https://voronov.nl";

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const COOKIE_NAME: &str = "three";


#[tokio::main]
async fn main() {
    let hello =
        warp::any()
            .and(filters::cookie::optional(COOKIE_NAME))
            .map(|counter: Option<u32>| {
                let counter = counter.unwrap_or(0) + 1;

                let counter_html: String = format!("{:0>8}", counter)
                    .chars()
                    .map(|c| format!("<span>{}</span> ", c))
                    .collect();

                let degrees = (counter % 360) as f32 / 10.0;

                let html = format!(
                    r#"
<title>Voronov</title>

<style>
html, body {{
    padding: 0;
    margin: 0;
    width: 100%;
    height: 100%;
}}

body {{
    background: lime;
    color: black;
    white-space: nowrap;
    overflow: hidden;
    font-family: Comic Sans, sans-serif;
    text-align: center;
}}

marquee {{
    font-family: serif;
    font-size: 96px;
    font-style: italic;
    margin-top: 100px;
}}

marquee span {{
    background: yellow;
}}

.construction {{
    margin-top: 50px;
    font-size: 25px;
}}

div {{
    position: absolute;
    bottom: 20px;
    left: 0;
    width: 100%;
}}

.message {{
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%) rotate({}deg);
    font-size: 36px;
    font-family: serif;
}}

.counter span {{
    display: inline-block;
    background: black;
    color: white;
    font-weight: bold;
    padding: 5px 2px;
    border-radius: 2px;
}}

.guestbook {{
    text-align: center;
}}

a {{
    color: white;
}}
</style>

<marquee direction="right" scrollamount="50"><span>Voronov's personal website</span></marquee>

<p class="message">
    Hi! And welcome to <strong><em>MY WEBSITE</em></strong>!!!!
</p>

<div>
    <p class="counter">
    {}
    </p>

    <p class="guestbook"><a href="https://voronov.nl">Guestbook</a></p>

    <p class="construction">🚧 🚧 🚧</p>
</div>
"#,
                    degrees, counter_html
                );

                warp::http::response::Builder::new()
                    .header(CONTENT_TYPE, "text/html; charset=utf-8")
                    .header(SET_COOKIE, format!("{}={}", COOKIE_NAME, counter))
                    .header(CACHE_CONTROL, "no-store")
                    .header(REFRESH, REFRESH_TARGET)
                    .status(200)
                    .body(html)
                    .unwrap()
            });

    warp::serve(hello).run((IP, PORT)).await;
}
