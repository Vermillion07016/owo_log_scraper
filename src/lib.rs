use std::fs;

use url::Url;

const BATTLE_FILE: &'static str = "battle_list.txt";

/// Read battle_list
pub fn load_battle_list() -> Result<Vec<Url>, Box<dyn std::error::Error>> {
    let file = fs::read_to_string(BATTLE_FILE)?;
    let file = file.trim();

    if file.is_empty() {
        return Err("Battle file is empty!".into());
    }

    let mut list = Vec::new();
    for line in file.lines() {
        let line = line.trim();

        let Ok(url) = Url::parse(line) else {
            eprintln!("Not a real url");
            continue;
        };
        list.push(url);
    }

    Ok(list)
}

/// Convert string to a f64
pub fn convert_string_to_f64(input: &str) -> Result<f64, String> {
    if !input.contains('%') {
        return Err("String yüzde işareti (%) içermiyor!".to_string());
    }

    input
        .trim()
        .trim_end_matches('%')
        .trim() // İşaret gittikten sonra kalabilecek olası boşluklar için
        .parse::<f64>()
        .map(|sayi| sayi / 100.0)
        .map_err(|e| format!("Sayıya dönüştürülemedi: {}", e))
}