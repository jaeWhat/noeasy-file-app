use std::fs::File;
use std::io::{
    Write,
    BufRead,
    BufReader
};
use std::path::Path;
use walkdir::WalkDir;

// const ROOT_DIR: &str = "../..";
const ROOT_DIR: &str = "../files";

#[tauri::command]
fn get_files(path: &str) -> Vec<String> {
    if !path.is_empty() {
        fn_get_file_list(path)
    } else {
        fn_get_file_list(ROOT_DIR)
    }
}

fn fn_get_file_list(dir_path: &str) -> Vec<String> {
    let mut entry_vec: Vec<String> = Vec::new();

    for entry in WalkDir::new(dir_path).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_dir() {
            println!("디렉토리: {}", entry.path().display());
        } else if entry.file_type().is_file() {
            println!("파일: {}", entry.path().display());
        }

        entry_vec.push(entry.path().display().to_string());
    }
    entry_vec
}

#[tauri::command]
fn create_file(file_name: &str) -> () {
    if !file_name.is_empty() {
        let new_file_path = format!("{}.txt", ROOT_DIR.to_owned() + "/" + file_name);

        let mut file = File::create(&new_file_path).expect("파일 생성 실패");
        file.write_all(b"Hello, this is a new file!\n").expect("쓰기 실패");
        println!("새 파일 생성됨: {}", new_file_path);
    } else {
        println!("빈 문자열입니다!");
    }
}

#[tauri::command]
fn read_file(file_name: &str) -> Vec<String> {
    let mut entry_vec: Vec<String> = Vec::new();

    if file_name.is_empty() {
        println!("빈 문자열입니다!");
        return entry_vec;
    }

    let target_file = format!("{}", fn_get_filename(file_name));

    for entry in WalkDir::new(ROOT_DIR).into_iter().filter_map(Result::ok) {
        let file_path = entry.path();

        // 파일인지 확인하고 파일 이름 비교
        if entry.file_type().is_file() && entry.file_name().to_string_lossy() == target_file {
            println!("찾은 파일: {:?}", file_path.display());

            if let Ok(file) = File::open(file_path) {
                let reader = BufReader::new(file);
                
                for line in reader.lines().filter_map(Result::ok) {
                    entry_vec.push(line);
                }
            } else {
                println!("파일을 열 수 없습니다: {:?}", file_path);
            }
        }
    }

    entry_vec
}

fn fn_get_filename(path: &str) -> &str {
    Path::new(path)
        .file_name() // 파일 이름 가져오기
        .and_then(|s| s.to_str()) // OsStr -> &str 변환
        .unwrap_or(path) // 변환 실패 시 전체 문자열 반환
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_files,
            create_file,
            read_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
