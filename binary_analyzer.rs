use crate::config_parser::Config;

#[derive(Debug)]
struct VideoHeader {
    width: u32,
    height: u32,
    frame_rate: f32,
}

pub fn analyze_binary(data: &Vec<u8>, config: &Config) {
    println!("Analyzing binary data with config: {:?}", config);
    println!("Binary data: {:?}", data);

    let video_header = parse_video_header(data);

    match video_header {
        Some(header) => {
            println!("Video Header Information:");
            println!("Width: {}", header.width);
            println!("Height: {}", header.height);
            println!("Frame Rate: {:.2} fps", header.frame_rate);
        }
        None => {
            eprintln!("Error parsing video header.");
        }
    }
}

fn parse_video_header(data: &Vec<u8>) -> Option<VideoHeader> {
    let width = u32::from_le_bytes(data[24..28].try_into().unwrap_or_default());
    let height = u32::from_le_bytes(data[28..32].try_into().unwrap_or_default());
    let frame_rate = f32::from_le_bytes(data[32..36].try_into().unwrap_or_default());

    Some(VideoHeader {
        width,
        height,
        frame_rate,
    })
}
