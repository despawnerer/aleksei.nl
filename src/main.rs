use rand::Rng;
use warp::filters;
use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH, SET_COOKIE};
use warp::Filter;

const REFRESH_TARGET: &str = "2;https://voronov.nl";

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const COOKIE_NAME: &str = "five";

const GREETINGS: [&'static str; 12] = [
    "Hi! I'm Mr. Voronov and this is my website!",
    "Hoi! Ik ben meneer Voronov en dit is mijn website!",
    "Привет! Я — г-н Воронов и это мой вебсайт!",
    "¡Hola! ¡Soy el Sr. Voronov y este es mi sitio web!",
    "أهلا! أنا السيد فورونوف وهذا موقع الويب الخاص بي!",
    "Привіт! Я пан Воронов, і це мій веб-сайт!",
    "<small>Help I am trapped in a website factory</small>",
    "Salve! Dominus ego Voronov sum et hoc website meum est!",
    "<small>Seriously, please help me</small>",
    "Ciao! Sono il signor Voronov e questo è il mio sito web!",
    "<small>This is not a joke, nobody knows I'm here</small>",
    "Salut! Je suis M. Voronov et ceci est mon website!",
];

#[tokio::main]
async fn main() {
    let hello =
        warp::any()
            .and(filters::cookie::optional(COOKIE_NAME))
            .map(|maybe_counter: Option<usize>| {

                let mut greeting_number = maybe_counter.unwrap_or(0);

                if greeting_number >= GREETINGS.len() {
                    greeting_number = 0;
                }

                let html =
                    format!(r#"
<title>Voronov</title>

<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=PT+Sans&display=swap" rel="stylesheet">

<style>
html, body {{
    padding: 0;
    margin: 0;
    width: 100%;
    height: 100%;
}}

body {{
    background: #8A2D1C;
    color: white;
    overflow: hidden;
    text-align: center;
    font-size: 5vh;
    font-family: 'PT Sans', sans-serif;
    line-height: 5vh;
}}

div {{
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    max-width: 100%;
}}

small {{
    font-size: 50%;
    opacity: 50%;
}}
</style>


<div>{}</div>
"#, GREETINGS[greeting_number]);

                greeting_number += 1;

                warp::http::response::Builder::new()
                    .header(CONTENT_TYPE, "text/html; charset=utf-8")
                    .header(SET_COOKIE, format!("{}={}", COOKIE_NAME, greeting_number.to_string()))
                    .header(CACHE_CONTROL, "no-store")
                    .header(REFRESH, REFRESH_TARGET)
                    .status(200)
                    .body(html)
                    .unwrap()
            });

    warp::serve(hello).run((IP, PORT)).await;
}
