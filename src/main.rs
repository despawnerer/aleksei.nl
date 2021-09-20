use rand::Rng;
use warp::filters;
use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH, SET_COOKIE};
use warp::Filter;

const REFRESH_TARGET: &str = "1;https://voronov.nl";

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const COOKIE_NAME: &str = "four";
const MAX_CRITTERS: usize = 30;

struct Critter {
    x: f32,
    y: f32,
}

fn deserialize_critters(string: String) -> Vec<Critter> {
    string
        .split("|")
        .filter_map(|critter_str| {
            let mut values_str = critter_str.split(',');

            let x = values_str.next().and_then(|x_str| x_str.parse().ok());
            let y = values_str.next().and_then(|y_str| y_str.parse().ok());

            let x = match x {
                None => return None,
                Some(x) => x,
            };

            let y = match y {
                None => return None,
                Some(y) => y,
            };

            if let Some(_) = values_str.next() {
                return None;
            }

            Some(Critter { x, y })
        })
        .collect()
}

fn serialize_critters(critters: &Vec<Critter>) -> String {
    let mut string = String::new();

    let mut critters_iter = critters
        .into_iter()
        .map(|critter| format!("{},{}", critter.x, critter.y));

    match critters_iter.next() {
        Some(critter_str) => string.push_str(&critter_str),
        None => return string,
    };

    for critter_str in critters_iter {
        string.push('|');
        string.push_str(&critter_str);
    }

    string
}

#[tokio::main]
async fn main() {
    let hello =
        warp::any()
            .and(filters::cookie::optional(COOKIE_NAME))
            .map(|serialized_critters: Option<String>| {
                let mut rng = rand::thread_rng();

                let mut critters = serialized_critters
                    .map(deserialize_critters)
                    .unwrap_or(Vec::new());

                critters.push(Critter {
                    x: rng.gen(),
                    y: rng.gen(),
                });

                let mut html =
                    r#"
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
    white-space: nowrap;
    overflow: hidden;
    text-align: center;
}

.critter {
    position: absolute;
    padding: 20px;
    border: 2px solid black;
    border-radius: 15px;
    background: white;
}

.eye {
    width: 50px;
    height: 50px;
    border: 2px solid black;
    border-radius: 50%;
    overflow: hidden;
    background: white;
    display: inline-block;
    position: relative;
}

.pupil {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    position: absolute;
    left: 15px;
    top: 0;
    background: black;
}

.mouth {
    position: absolute;
    bottom: 10px;
    left: 50%;
    font-family: sans-serif;
    background: black;
    border-radius: 50% 50% 0 0;
    width: 30px;
    height: 10px;
    transform: translateX(-50%);
}

.voronov {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    font-size: 10vh;
    text-align: center;
}

.voronov small {
    display: block;
    font-size: 25%;
}
</style>

<script>
let eyes = document.getElementsByClassName("eye");

document.onmousemove = function(event) {
    for (eye of eyes) {
        let eyeX = eye.getBoundingClientRect().left;
        let eyeY = eye.getBoundingClientRect().top;
        let radianDegrees = Math.atan2(event.pageX - eyeX - 25, event.pageY - eyeY - 25);
        let rotationDegrees = radianDegrees * (180 / Math.PI) * -1 + 180;
        eye.style.transform = `rotate(${rotationDegrees}deg)`;
    }
};
</script>

<div class="voronov">
    (the) Voronov
    <small>is watching you</small>
</div>
"#.to_string();

                for critter in &critters {
                    html.push_str(
                        &format!(
                            "<div class=\"critter\" style=\"left: {}%; top: {}%\"><div class=\"eye\"><div class=\"pupil\"></div></div> <div class=\"eye\"><div class=\"pupil\"></div></div><div class=\"mouth\"></div></div>",
                            critter.x * 100.0,
                            critter.y * 100.0
                        )
                    );
                }

                if critters.len() >= MAX_CRITTERS {
                    critters = Vec::new();
                }

                warp::http::response::Builder::new()
                    .header(CONTENT_TYPE, "text/html; charset=utf-8")
                    .header(SET_COOKIE, format!("{}={}", COOKIE_NAME, serialize_critters(&critters)))
                    .header(CACHE_CONTROL, "no-store")
                    .header(REFRESH, REFRESH_TARGET)
                    .status(200)
                    .body(html)
                    .unwrap()
            });

    warp::serve(hello).run((IP, PORT)).await;
}
