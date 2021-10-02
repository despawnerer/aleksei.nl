use rand::{
    rngs::ThreadRng,
    seq::SliceRandom,
    Rng,
};
use warp::filters;
use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH, SET_COOKIE};
use warp::Filter;

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const REFRESH_TARGET: &str = "1;https://voronov.nl";

const COOKIE_NAME: &str = "eight";
const MAX_ZALGO: u8 = 60;

const TEXT: &str = "Voronov";

#[tokio::main]
async fn main() {
    let hello =
        warp::path::end()
            .and(filters::cookie::optional(COOKIE_NAME))
            .map(|zalgo_amount: Option<u8>| {
                let zalgo_amount = zalgo_amount.unwrap_or(0);

                let zalogified_text_verytop = zalgoify(TEXT, zalgo_amount);
                let zalogified_text_top = zalgoify(TEXT, zalgo_amount);
                let zalogified_text_middle = zalgoify(TEXT, zalgo_amount);
                let zalogified_text_bottom = zalgoify(TEXT, zalgo_amount);
                let zalogified_text_verybottom = zalgoify(TEXT, zalgo_amount);

                let html =
                    format!(r#"
<!DOCTYPE html>

<title>Voronov</title>

<style type="text/css">

html {{
    padding: 0;
    margin: 0;
}}

body {{
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
    font-size: 18vh;
    font-family: serif;
    overflow: hidden;
}}

div {{
    position: absolute;
    left: 50%;
    transform: translate(-50%, -50%);
}}

.verytop {{
    top: 0;
}}

.top {{
    top: 25%;
}}

.middle {{
    top: 50%;
}}

.bottom {{
    top: 75%;
}}

.verybottom {{
    top: 100%;
}}
</style>

<div class="verytop">{}</div>
<div class="top">{}</div>
<div class="middle">{}</div>
<div class="bottom">{}</div>
<div class="verybottom">{}</div>
"#, zalogified_text_verytop, zalogified_text_top, zalogified_text_middle, zalogified_text_bottom, zalogified_text_verybottom);

                let next_zalgo_amount = if zalgo_amount >= MAX_ZALGO { 0 } else { zalgo_amount + 1 };

                warp::http::response::Builder::new()
                    .header(CONTENT_TYPE, "text/html; charset=utf-8")
                    .header(SET_COOKIE, format!("{}={}", COOKIE_NAME, next_zalgo_amount))
                    .header(CACHE_CONTROL, "no-store")
                    .header(REFRESH, REFRESH_TARGET)
                    .status(200)
                    .body(html)
                    .unwrap()
            });

    warp::serve(hello).run((IP, PORT)).await;
}

fn zalgoify(input: &str, amount: u8) -> String {
    let mut rng = rand::thread_rng();

    let cap = input.len() + input.len() * amount as usize * 3;

    let mut ret = String::with_capacity(cap);

    for _ in 0..amount {
        ret.push_str("&nbsp;");
        ret.push(*chars::ZALGO_UP.choose(&mut rng).unwrap());
        ret.push(*chars::ZALGO_MID.choose(&mut rng).unwrap());
        ret.push(*chars::ZALGO_DOWN.choose(&mut rng).unwrap());
    }

    for c in input.chars() {
        ret.push(c);

        for _ in 0..amount {
            ret.push(*chars::ZALGO_UP.choose(&mut rng).unwrap());
        }

        for _ in 0..amount {
            ret.push(*chars::ZALGO_MID.choose(&mut rng).unwrap());
        }

        for _ in 0..amount {
            ret.push(*chars::ZALGO_DOWN.choose(&mut rng).unwrap());
        }
    }

    for _ in 0..amount {
        ret.push_str("&nbsp;");
        ret.push(*chars::ZALGO_UP.choose(&mut rng).unwrap());
        ret.push(*chars::ZALGO_MID.choose(&mut rng).unwrap());
        ret.push(*chars::ZALGO_DOWN.choose(&mut rng).unwrap());
    }

    ret
}

mod chars {
    pub const ZALGO_UP: &[char] = &[
        '\u{030d}', // ̍
        '\u{030e}', // ̎
        '\u{0304}', // ̄
        '\u{0305}', // ̅
        '\u{033f}', // ̿
        '\u{0311}', // ̑
        '\u{0306}', // ̆
        '\u{0310}', // ̐
        '\u{0352}', // ͒
        '\u{0357}', // ͗
        '\u{0351}', // ͑
        '\u{0307}', // ̇
        '\u{0308}', // ̈
        '\u{030a}', // ̊
        '\u{0342}', // ͂
        '\u{0343}', // ̓
        '\u{0344}', // ̈́
        '\u{034a}', // ͊
        '\u{034b}', // ͋
        '\u{034c}', // ͌
        '\u{0303}', // ̃
        '\u{0302}', // ̂
        '\u{030c}', // ̌
        '\u{0350}', // ͐
        '\u{0300}', // ̀
        '\u{0301}', // ́
        '\u{030b}', // ̋
        '\u{030f}', // ̏
        '\u{0312}', // ̒
        '\u{0313}', // ̓
        '\u{0314}', // ̔
        '\u{033d}', // ̽
        '\u{0309}', // ̉
        '\u{0363}', // ͣ
        '\u{0364}', // ͤ
        '\u{0365}', // ͥ
        '\u{0366}', // ͦ
        '\u{0367}', // ͧ
        '\u{0368}', // ͨ
        '\u{0369}', // ͩ
        '\u{036a}', // ͪ
        '\u{036b}', // ͫ
        '\u{036c}', // ͬ
        '\u{036d}', // ͭ
        '\u{036e}', // ͮ
        '\u{036f}', // ͯ
        '\u{033e}', // ̾
        '\u{035b}', // ͛
        '\u{0346}', // ͆
        '\u{031a}', // ̚
    ];

    pub const ZALGO_DOWN: &[char] = &[
        '\u{0316}', // ̖
        '\u{0317}', // ̗
        '\u{0318}', // ̘
        '\u{0319}', // ̙
        '\u{031c}', // ̜
        '\u{031d}', // ̝
        '\u{031e}', // ̞
        '\u{031f}', // ̟
        '\u{0320}', // ̠
        '\u{0324}', // ̤
        '\u{0325}', // ̥
        '\u{0326}', // ̦
        '\u{0329}', // ̩
        '\u{032a}', // ̪
        '\u{032b}', // ̫
        '\u{032c}', // ̬
        '\u{032d}', // ̭
        '\u{032e}', // ̮
        '\u{032f}', // ̯
        '\u{0330}', // ̰
        '\u{0331}', // ̱
        '\u{0332}', // ̲
        '\u{0333}', // ̳
        '\u{0339}', // ̹
        '\u{033a}', // ̺
        '\u{033b}', // ̻
        '\u{033c}', // ̼
        '\u{0345}', // ͅ
        '\u{0347}', // ͇
        '\u{0348}', // ͈
        '\u{0349}', // ͉
        '\u{034d}', // ͍
        '\u{034e}', // ͎
        '\u{0353}', // ͓
        '\u{0354}', // ͔
        '\u{0355}', // ͕
        '\u{0356}', // ͖
        '\u{0359}', // ͙
        '\u{035a}', // ͚
        '\u{0323}', // ̣
    ];

    pub const ZALGO_MID: &[char] = &[
        '\u{0315}', // ̕
        '\u{031b}', // ̛
        '\u{0340}', // ̀
        '\u{0341}', // ́
        '\u{0358}', // ͘
        '\u{0321}', // ̡
        '\u{0322}', // ̢
        '\u{0327}', // ̧
        '\u{0328}', // ̨
        '\u{0334}', // ̴
        '\u{0335}', // ̵
        '\u{0336}', // ̶
        '\u{034f}', // ͏
        '\u{035c}', // ͜
        '\u{035d}', // ͝
        '\u{035e}', // ͞
        '\u{035f}', // ͟
        '\u{0360}', // ͠
        '\u{0362}', // ͢
        '\u{0338}', // ̸
        '\u{0337}', // ̷
        '\u{0361}', // ͡
        '\u{0489}', // ҉_
    ];
}
