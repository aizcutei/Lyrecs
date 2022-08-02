
pub fn time_f64_to_time_tag(time: f64) -> String {
    let time_mm = (time.floor()/60.0).floor();
    let time_ss = time.floor()%60.0;
    let time_ms = time.fract()*1000.0;
    format!("{}{}:{}.{}", (time_mm as i64)/10, (time_mm as i64) %10, time_ss, time_ms.floor())
}

pub fn time_tag_to_time_f64(time_tag: &str) -> f64 {
    let time_str = time_tag.split(":").collect::<Vec<&str>>();
    let time_mm = time_str[0].parse::<f64>().unwrap();
    let time_ss = time_str[1].parse::<f64>().unwrap();
    time_mm*60.0 + time_ss
}