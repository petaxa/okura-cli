use clap::Parser;
use std::{fs, process::Command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 解析対象の画像ファイルパス
    #[arg(short, long)]
    image: String,

    /// 改行を無視するかどうか
    #[arg(short, long, default_value_t = true)]
    line_break: bool,
}

fn main() {
    let args = Args::parse();
    let output_path = "results/_image_p1.md";

    println!("Hello!");

    clear_output(&output_path);

    run_yomitoku(&args.image, args.line_break);

    if is_success_ocr(&output_path) {
        remove_image(&args.image);
    }
}

/**
 * 出力先ファイルの中身を空にする
 */
fn clear_output(output_path: &str) {
    fs::write(output_path, "").expect("failed overwrite output file");
}

/**
 * YomiToku を実行する
 */
fn run_yomitoku(image_path: &str, is_line_break: bool) {
    let mut command = Command::new("yomitoku");
    command.arg(image_path).arg("-f").arg("md");

    if is_line_break {
        command.arg("--ignore_line_break");
    }

    command.status().expect("OCR failed");
}

/**
 * OCR の成否を確認する
 */
fn is_success_ocr(output_path: &str) -> bool {
    let metadata = fs::metadata(output_path).expect("Failed to get file metadata");
    return metadata.len() > 0;
}

/**
 * 画像ファイルを削除する
 */
fn remove_image(image_path: &str) {
    fs::remove_file(image_path).expect("failed delete image file");
}
