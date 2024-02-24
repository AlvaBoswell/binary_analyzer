mod file_reader;
mod binary_analyzer;
mod config_parser;

fn main() {
    let config = config_parser::parse_config("config.toml");
    let file_contents = file_reader::read_file("data.bin");
    binary_analyzer::analyze_binary(&file_contents, &config);
}
