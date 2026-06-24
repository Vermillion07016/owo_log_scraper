use std::{collections::HashMap, fs, sync::Arc, time::{Duration, Instant}};
use headless_chrome::{Browser, Tab};
use owo_log_scrape::load_battle_list;
use tokio::sync::mpsc::UnboundedSender;
use url::Url;
use crate::structs::Pet;
mod structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let battle_list = load_battle_list()?;

    let browser = Browser::default()?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    for url in battle_list {
        // opens new tab
        let tab = browser.new_tab()?;

        tokio::spawn(process_log(tab, url, tx.clone()));
    }
    let mut index = 1;
    while let Some(data) = rx.recv().await {
        fs::write(format!("battlelog_{}", index), data)?;
        index += 1;
    }

    Ok(())
}
async fn process_log(tab: Arc<Tab>, url: Url, tx: UnboundedSender<String>) {
    // navigate to the url
    tab.navigate_to(url.as_str()).unwrap();
    std::thread::sleep(Duration::from_secs(2));
    
    let base = js_dom::js_wait_global(&tab, "div[class*='log-card'] > span", 2000, 200).unwrap();
    let base_id = base.object_id.unwrap_or_default();

    let pets = load_pets(&tab, &base_id).unwrap();
    let battles = load_battles(&tab, &base_id).unwrap();

    let mut pet_txt = serde_json::to_string_pretty(&pets).unwrap();
    let battle_txt = serde_json::to_string_pretty(&battles).unwrap();

    pet_txt.push_str(" \n\n");
    pet_txt.push_str(&battle_txt);
    drop(battle_txt);

    tx.send(pet_txt).unwrap();
}
fn load_pets(tab: &Arc<Tab>, base_id: &str) -> Result<Vec<Pet>, Box<dyn std::error::Error>> {
    let pet_infos = js_dom::js_wait_all(tab, base_id, "div.pet-info", 6, 10000, 200)?;
    if pet_infos.len() <= 0 { return Err("couldnt found pet-infos".into()); }

    println!("founded pets: {}", pet_infos.len());
    
    let mut pets = Vec::new();
    let total_time = Instant::now();
    for pet_object in pet_infos {
        let id = pet_object.object_id.as_deref().unwrap_or_default();

        let dur = Instant::now();
        let pet = Pet::new(id, &tab)?;
        println!("parsing duration: {:?}", dur.elapsed());

        pets.push(pet);
    }
    println!("total loading pet time is: {:?}", total_time.elapsed());

    Ok(pets)
}
fn load_battles(tab: &Arc<Tab>, base_id: &str) -> Result<HashMap<u8, String>, Box<dyn std::error::Error>> {
    let pages_obj = js_dom::js_find(tab, base_id, "div.display > div.pages > div")?;
    let pages_id =pages_obj.object_id.as_deref().unwrap_or_default();
    let pages_txt = js_dom::js_text(tab, pages_id)?;

    let Some((page_num, max_page)) = pages_txt.split_once("/") else {
        return Err("Pages bulunamadi".into());
    };
    let page_num = page_num.trim().parse::<u8>()?;
    let max_page = max_page.trim().parse::<u8>()?;

    let btn = js_dom::js_find_all(tab, base_id, "div.display > div.pages > button.v-btn")?;
    if btn.len() < 2 {
        return Err("pages buttonlari bulunamadi".into());
    }   
    let btn_id = btn[1].object_id.as_deref().unwrap_or_default();

    let mut logs = HashMap::new();
    for num in page_num+1..=max_page {
        js_dom::js_click(tab, btn_id)?;

        let log_objs = js_dom::js_find_all(tab, base_id, "div.log-box > div > div.log-row")?;
        if log_objs.is_empty() { continue; }
        
        let mut row = String::new();
        let mut i = 0;
        for log in log_objs {
            let log_id = log.object_id.as_deref().unwrap_or_default();

            let text = js_dom::js_text(tab, log_id)?;
            row.push_str(&text.trim());
            row.push_str(" ");

            if i == 5 {
                i = 0;

                logs.insert(num-1, row.clone());
                row.clear();
            }
            i+= 1;
        }
    }

    Ok(logs)
}