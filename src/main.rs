use warp::filters;
use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH, SET_COOKIE};
use warp::Filter;

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const REFRESH_TARGET: &str = "1;https://voronov.nl";

const COOKIE_NAME: &str = "seven";
const MAX_BLUR: u8 = 60;

#[tokio::main]
async fn main() {
    let hello =
        warp::path::end()
            .and(filters::cookie::optional(COOKIE_NAME))
            .map(|blur: Option<u8>| {
                let blur = blur.unwrap_or(0);

                let html =
                    format!(r#"
<!DOCTYPE html>

<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Bree+Serif&display=swap" rel="stylesheet">

<title>Voronov</title>

<style type="text/css">

html {{
    padding: 0 2vw;
    margin: 0;
}}

body {{
    width: 100%;
    height: 100%;
    max-width: 600px;
    margin: 0 auto;
    padding: 0;
    font-size: 18vh;
    font-family: 'Bree Serif', serif;
    position: relative;
    filter: blur({}px);
}}

div {{
    position: absolute;
}}

.letter-1-1 {{
    left: 50%;
    transform: translate(-50%);
    top: 1vh;
}}

.letter-2-1, .letter-2-2 {{
    top: 16vh;
    font-size: 80%;
}}

.letter-2-1 {{ left: 0; }}
.letter-2-2 {{ right: 0; }}

.letter-3-1, .letter-3-2, .letter-3-3 {{
    top: 31vh;
    font-size: 60%;
}}

.letter-3-1 {{ left: 0; }}
.letter-3-2 {{ left: 50%; transform: translate(-50%); }}
.letter-3-3 {{ right: 0; }}


.letter-4-1, .letter-4-2, .letter-4-3, .letter-4-4 {{
    top: 46vh;
    font-size: 40%;
}}

.letter-4-1 {{ left: 0; }}
.letter-4-2 {{ left: 33.33%; transform: translate(-50%); }}
.letter-4-3 {{ left: 66.66%; transform: translate(-50%); }}
.letter-4-4 {{ right: 0; }}


.letter-5-1, .letter-5-2, .letter-5-3, .letter-5-4, .letter-5-5 {{
    top: 61vh;
    font-size: 40%;
}}

.letter-5-1 {{ left: 0; }}
.letter-5-2 {{ left: 25%; transform: translate(-50%); }}
.letter-5-3 {{ left: 50%; transform: translate(-50%); }}
.letter-5-4 {{ left: 75%; transform: translate(-50%); }}
.letter-5-5 {{ right: 0; }}



.letter-6-1, .letter-6-2, .letter-6-3, .letter-6-4, .letter-6-5, .letter-6-6 {{
    top: 76vh;
    font-size: 20%;
}}

.letter-6-1 {{ left: 0; }}
.letter-6-2 {{ left: 20%; transform: translate(-50%); }}
.letter-6-3 {{ left: 40%; transform: translate(-50%); }}
.letter-6-4 {{ left: 60%; transform: translate(-50%); }}
.letter-6-5 {{ left: 80%; transform: translate(-50%); }}
.letter-6-6 {{ right: 0; }}


.letter-7-1, .letter-7-2, .letter-7-3, .letter-7-4, .letter-7-5, .letter-7-6, .letter-7-7 {{
    top: 91vh;
    font-size: 10%;
}}

.letter-7-1 {{ left: 0; }}
.letter-7-2 {{ left: 16.66%; transform: translate(-50%); }}
.letter-7-3 {{ left: 33.33%; transform: translate(-50%); }}
.letter-7-4 {{ left: 50%; transform: translate(-50%); }}
.letter-7-5 {{ left: 66.66%; transform: translate(-50%); }}
.letter-7-6 {{ left: 83.33%; transform: translate(-50%); }}
.letter-7-7 {{ right: 0; }}

</style>

<div class="letter-1-1">V</div>

<div class="letter-2-1">O</div>
<div class="letter-2-2">R</div>

<div class="letter-3-1">O</div>
<div class="letter-3-2">N</div>
<div class="letter-3-3">O</div>

<div class="letter-4-1">V</div>
<div class="letter-4-2">A</div>
<div class="letter-4-3">L</div>
<div class="letter-4-4">E</div>

<div class="letter-5-1">K</div>
<div class="letter-5-2">S</div>
<div class="letter-5-3">E</div>
<div class="letter-5-4">I</div>
<div class="letter-5-5">V</div>

<div class="letter-6-1">O</div>
<div class="letter-6-2">R</div>
<div class="letter-6-3">O</div>
<div class="letter-6-4">N</div>
<div class="letter-6-5">O</div>
<div class="letter-6-6">V</div>

<div class="letter-7-1">A</div>
<div class="letter-7-2">L</div>
<div class="letter-7-3">E</div>
<div class="letter-7-4">K</div>
<div class="letter-7-5">S</div>
<div class="letter-7-6">E</div>
<div class="letter-7-7">I</div>
"#, blur);

                let next_blur = if blur >= MAX_BLUR { 0 } else { blur + 1 };

                warp::http::response::Builder::new()
                    .header(CONTENT_TYPE, "text/html; charset=utf-8")
                    .header(SET_COOKIE, format!("{}={}", COOKIE_NAME, next_blur))
                    .header(CACHE_CONTROL, "no-store")
                    .header(REFRESH, REFRESH_TARGET)
                    .status(200)
                    .body(html)
                    .unwrap()
            });

    warp::serve(hello).run((IP, PORT)).await;
}
