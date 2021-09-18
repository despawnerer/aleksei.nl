use rand::Rng;
use warp::filters;
use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH, SET_COOKIE};
use warp::Filter;

const REFRESH_TARGET: &str = "1;https://voronov.nl";

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const COOKIE_NAME: &str = "two";
const MIN_FONT_SIZE: f32 = 16.0;
const MAX_FONT_SIZE: f32 = 300.0;
const VORONOVS_FOR_A_STAR: usize = 60;
const MAX_VORONOVS: usize = 80;

struct Voronov {
    x: f32,
    y: f32,
    font_size: f32,
}

fn deserialize_voronovs(string: String) -> Vec<Voronov> {
    string
        .split("|")
        .filter_map(|voronov_str| {
            let mut values_str = voronov_str.split(',');

            let x = values_str.next().and_then(|x_str| x_str.parse().ok());
            let y = values_str.next().and_then(|y_str| y_str.parse().ok());
            let font_size = values_str.next().and_then(|fs_str| fs_str.parse().ok());

            let x = match x {
                None => return None,
                Some(x) => x,
            };

            let y = match y {
                None => return None,
                Some(y) => y,
            };

            let font_size = match font_size {
                None => return None,
                Some(font_size) => font_size,
            };

            if let Some(_) = values_str.next() {
                return None;
            }

            Some(Voronov { x, y, font_size })
        })
        .collect()
}

fn serialize_voronovs(voronovs: &Vec<Voronov>) -> String {
    let mut string = String::new();

    let mut voronovs_iter = voronovs
        .into_iter()
        .map(|v| format!("{},{},{}", v.x, v.y, v.font_size));

    match voronovs_iter.next() {
        Some(voronov_str) => string.push_str(&voronov_str),
        None => return string,
    };

    for voronov_str in voronovs_iter {
        string.push('|');
        string.push_str(&voronov_str);
    }

    string
}

#[tokio::main]
async fn main() {
    let hello = warp::any().and(filters::cookie::optional(COOKIE_NAME)).map(
        |serialized_voronovs: Option<String>| {
            let mut rng = rand::thread_rng();

            let mut voronovs = serialized_voronovs
                .map(deserialize_voronovs)
                .unwrap_or(Vec::new());

            voronovs.push(Voronov {
                x: rng.gen(),
                y: rng.gen(),
                font_size: rng.gen_range(MIN_FONT_SIZE..MAX_FONT_SIZE),
            });

            let mut html = r#"
<title>Voronov</title>

<style>
html, body {
    padding: 0;
    margin: 0;
    width: 100%;
    height: 100%;
}

body {
    background: white;
    color: black;
    font-family: sans-serif;
    font-weight: bold;
    white-space: nowrap;
    overflow: hidden;
}

span {
    position: absolute;
    transform: translate(-50%, -50%);
}

.star {
    left: 50%;
    top: 50%;
    font-size: 150px;
}
</style>
"#
            .to_string();

            for voronov in &voronovs {
                html.push_str(&format!(
                    "<span style=\"left: {}%; top: {}%; font-size: {}px\">Voronov</span>",
                    voronov.x * 100.0,
                    voronov.y * 100.0,
                    voronov.font_size
                ));
            }

            if voronovs.len() >= VORONOVS_FOR_A_STAR {
                html.push_str("<span class=\"star\">⭐️</span>");
            }

            if voronovs.len() >= MAX_VORONOVS {
                voronovs = Vec::new();
            }

            warp::http::response::Builder::new()
                .header(CONTENT_TYPE, "text/html; charset=utf-8")
                .header(
                    SET_COOKIE,
                    format!("{}={}", COOKIE_NAME, serialize_voronovs(&voronovs)),
                )
                .header(CACHE_CONTROL, "no-store")
                .header(REFRESH, REFRESH_TARGET)
                .status(200)
                .body(html)
                .unwrap()
        },
    );

    warp::serve(hello).run((IP, PORT)).await;
}
