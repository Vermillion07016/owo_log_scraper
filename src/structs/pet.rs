use std::sync::Arc;
use headless_chrome::Tab;
use js_dom::{js_find_all, js_text, js_wait_global};
use owo_log_scrape::convert_string_to_f64;
use serde::Serialize;
use crate::structs::weapon::Weapon;

#[derive(Default, Debug, Serialize)]
#[allow(dead_code)]
pub struct Pet {
    pub name: String,
    pub level: u8,

    pub max_health: u16,
    pub health: u16,
    pub max_mana: u16,
    pub mana: u16,

    pub physical_attack: u16,
    pub magical_attack: u16,
    pub physical_resistance: f64,
    pub magical_resistance: f64,

    pub weapon: Option<Weapon>,
    pub passives: Vec<String>,

    object_id: String,
}

type Err = Box<dyn std::error::Error>;

impl Pet {
    pub fn new(object_id: &str, tab: &Arc<Tab>) -> Result<Self, Err> {
        let mut pet = Self::default();
        pet.object_id = object_id.to_owned();

        pet.update_health_and_mana(tab)?;
        pet.update_pet_name_and_lvl(tab)?;
        pet.update_resistance_values(tab)?;
        pet.update_weapon(tab)?;
        pet.update_passives(tab)?;

        Ok(pet)
    }

    fn update_pet_name_and_lvl(&mut self, tab: &Arc<Tab>) -> Result<(), Err> {
        let id = &self.object_id;

        let divs = js_dom::js_find_all(tab, id, "div.pet-text > div");
        if divs.is_err() {
            self.name = "Unknown".into();
            self.level = u8::MAX;
            return Ok(());
        }
        let divs = divs.unwrap();

        let texts: Vec<String> = divs.iter().map(|object| {
            let id = object.object_id.as_deref().unwrap();
            
            js_dom::js_text(tab, id).unwrap_or_default().trim().to_owned()
        }).collect();

        if let Some((_, lvl)) = texts[0].split_once(" ") {
            let lvl = lvl.parse::<u8>().unwrap_or(u8::MAX);

            self.level = lvl;
        };
        self.name = texts[1].to_owned();

        Ok(())
    }
    fn update_health_and_mana(&mut self, tab: &Arc<Tab>) -> Result<(), Err> {
        let id = &self.object_id;

        let bars = js_dom::js_find_all(tab, id, "div[class*='bar-text']")?;
        
        let points: Vec<(u16,u16)> = bars.iter().map(|bar| {
            let id = bar.object_id.as_deref().unwrap();
            let text = js_dom::js_text(tab, id).unwrap_or_default();

            if let Some((point, max_point)) = text.split_once("/") {
                let point: u16 = point.trim().parse().unwrap_or_default();
                let max_point: u16 = max_point.trim().parse().unwrap_or_default();

                return (point, max_point);
            }else {
                return (0,0);
            }
        }).collect();
        self.health = points[0].0;
        self.max_health = points[0].1;
        self.mana = points[1].0;
        self.max_mana = points[1].1;

        Ok(())
    }
    fn update_resistance_values(&mut self, tab: &Arc<Tab>) -> Result<(), Err> {
        let stats = js_find_all(tab, &self.object_id, "div.weapon-row div.stat-text")?;

        if stats.len() < 4 {
            return Err(format!("Coudlnt continue because cant find expected size of stats column. Expected size: 4, founded size: {}", stats.len()).into());
        }

        let text: Vec<String> = stats.iter().map(|object| {
            let id = object.object_id.as_deref().unwrap();

            js_dom::js_text(tab, id).unwrap_or_default().trim().to_owned()
        }).collect();

        self.physical_attack = text[0].parse()?;
        self.physical_resistance = convert_string_to_f64(&text[1])?;
        self.magical_attack = text[2].parse()?;
        self.magical_resistance = convert_string_to_f64(&text[3])?;
        
        Ok(())
    }
    fn update_weapon(&mut self, tab: &Arc<Tab>) -> Result<(), Err> {
        let id = &self.object_id;

        let button = js_dom::js_find(tab, id, "div.weapon-row img")?;
        let button_id = button.object_id.as_deref().unwrap_or_default();

        if js_dom::js_find(tab, button_id, "img").is_err() {
            return Ok(());
        }
        js_dom::js_click(tab, button_id)?;

        let title = js_wait_global(tab, "div[class*='v-dialog__content--active'] div[class*='v-list-item__title']", 5000, 100)?;
        let title_id = title.object_id.as_deref().unwrap_or_default();

        let title = js_text(tab, title_id)?.trim().to_owned();

        let selector = "div[class*='v-dialog__content--active'] div.v-list-item__content div.v-list-item__content > :is(div, span)";
        let divs = js_dom::js_find_all_global(tab, selector)?;

        let texts: Vec<String> = divs.iter().map(|object| {
            let id = object.object_id.as_deref().unwrap_or_default();

            js_dom::js_text(tab, id).unwrap_or_default().trim().to_owned()
        }).collect();

        let weapon = Weapon::new(title, &texts)?;
        self.weapon = Some(weapon);

        let js_code = r#"
        function click() {
            const el = document.querySelector("div.v-overlay--active");
            if (!el) return ;

            const cfg = { view: window, clientX: 900, clientY: 100, screenX: 900, screenY: 98, button: 0, buttons: 1, bubbles: true, cancelable: true };
            
            ['mousedown', 'mouseup', 'click'].forEach(type => el.dispatchEvent(new MouseEvent(type, cfg)));
        }
        click()"#;

        tab.evaluate(js_code, false)?;

        let wait_js = r#"
        new Promise(resolve => {
            const check = () => {
                if (!document.querySelector("div.v-overlay--active")) {
                    resolve(true);
                } else {
                    setTimeout(check, 50);
                }
            };
            check();
        })"#;

        tab.evaluate(wait_js, true)?;
        
        Ok(())
    }
    fn update_passives(&mut self, tab: &Arc<Tab>) -> Result<(), Err> {
        let id = &self.object_id;

        let selector = "div.weapon-row > div.stat-col > div[class*='passive-icon']";
        let _passives = js_dom::js_find_all(tab, id, selector)?;

        // yapilacak
        
        Ok(())
    }
}