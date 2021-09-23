use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH};
use warp::Filter;

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const FRAMES: &'static [Frame] = &[
    Frame::Dialogue("Hi", 2),
    Frame::Dialogue("Can I ask you a question?", 2),
    Frame::Dialogue("What do you do for a living?", 2),
    Frame::Dialogue("Do you like it?", 2),
    Frame::Dialogue("What would you like to do instead?", 2),
    Frame::Dialogue("Why don’t you try?", 2),
    Frame::Dialogue("Would you like me to show you?", 2),
    Frame::Dialogue("Here you go:", 2),
    Frame::Dialogue("Nice, isn’t it? Let’s do that again:", 3),
    Frame::Dialogue("So lovely. Do you want try now?", 3),
    Frame::Dialogue("We’re all afraid. Do it anyway.", 3),
    Frame::Art("really-badly-drawn-cat.jpg"),
    Frame::Dialogue("I told you. Now keep doing it.", 2),
    Frame::Art("weirder-cat-2.jpg"),
    Frame::Art("weirder-cat-4.jpg"),
    Frame::Dialogue("Wait, no, not like that", 2),
    Frame::Dialogue("Okay this is getting weird now.<br/>Shut it down.<br/>Shut it down.", 2),
];

enum Frame {
    Dialogue(&'static str, u8),
    Art(&'static str),
    End,
}

#[tokio::main]
async fn main() {
    let init =
        warp::path::end()
            .map(|| {
                warp::http::response::Builder::new()
                    .header(CONTENT_TYPE, "text/html; charset=utf-8")
                    .header(CACHE_CONTROL, "no-store")
                    .header(REFRESH, "1;https://aleksei.nl/0")
                    .status(200)
                    .body("")
                    .unwrap()
            });

    let frame =
        warp::path::param::<usize>()
            .and(warp::path::end())
            .map(|n: usize| {
                let n = if n >= FRAMES.len() { 0 } else { n };

                let (frame_html, delay) = match FRAMES[n] {
                    Frame::Dialogue(text, delay) => (format!("<div class=\"dialogue\">{}</div>", text), delay),
                    Frame::Art(filename) => (format!("<img src=\"/resources/{}\"/>", filename), 3),
                    Frame::End => ("<div class=\"fin\">fin</div>".to_owned(), 5),
                };

                let html =
                    format!(r#"
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Playfair+Display&display=swap" rel="stylesheet">

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
    color: #222;
    overflow: hidden;
    text-align: center;
    font-size: 5vh;
    font-family: 'Playfair Display', serif;
    line-height: 5vh;
}}

.dialogue {{
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    max-width: 100%;
}}

img {{
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    height: 50vh;
}}
</style>

{}
"#, frame_html);

                let refresh_target = format!("{};https://voronov.nl/{}", delay, n);

                warp::http::response::Builder::new()
                    .header(CONTENT_TYPE, "text/html; charset=utf-8")
                    .header(CACHE_CONTROL, "no-store")
                    .header(REFRESH, refresh_target)
                    .status(200)
                    .body(html)
                    .unwrap()
            });

    let resources = warp::path("resources").and(warp::fs::dir("resources"));

    warp::serve(resources.or(init).or(frame)).run((IP, PORT)).await;
}
