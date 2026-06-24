# owo-log-scrape

Headless Chrome kullanarak web üzerindeki savaş loglarını otomatik olarak tarayan ve JSON formatında dışa aktaran bir Rust aracı.

## Ne Yapar

`battle_list.txt` dosyasındaki URL'leri okur, her URL için bir tarayıcı sekmesi açar ve sayfadaki şu verileri toplar:

- **Pet bilgileri:** İsim, seviye, can/mana, saldırı/direnç değerleri, silah ve pasifler
- **Savaş logları:** Sayfalandırılmış log satırları

Toplanan veriler `battlelog_1`, `battlelog_2`, ... şeklinde dosyalara yazılır.

## Proje Yapısı

```
src/
├── main.rs          # Giriş noktası, tarayıcı yönetimi ve async koordinasyon
├── lib.rs           # URL yükleme ve yardımcı fonksiyonlar
└── structs/
    ├── mod.rs
    ├── pet.rs       # Pet yapısı ve DOM'dan veri çekme
    ├── weapon.rs    # Silah yapısı
    └── rank.rs      # Silah kalitesi enum'u
```

## Gereksinimler

- Rust (stable)
- js_dom (benim lokal projem)
- Chrome / Chromium kurulu olmalı (headless_chrome tarafından kullanılır)

## Kullanım

1. `battle_list.txt` dosyası oluştur ve her satıra bir URL yaz:

```
https://example.com/battle/123
https://example.com/battle/456
```

2. Projeyi çalıştır:

```bash
cargo run --release
```

3. Her URL için `battlelog_1`, `battlelog_2` ... adında çıktı dosyaları oluşturulur. İçerik JSON formatındadır; önce pet listesi, ardından savaş logları gelir.

## Veri Formatı

### Pet

```json
{
  "name": "...",
  "level": 10,
  "max_health": 500,
  "health": 420,
  "max_mana": 200,
  "mana": 150,
  "physical_attack": 80,
  "magical_attack": 60,
  "physical_resistance": 0.15,
  "magical_resistance": 0.10,
  "weapon": { ... },
  "passives": []
}
```

### Silah Rankları

`Common` → `UnCommon` → `Rare` → `Epic` → `Mythical` → `Legendary`

## Notlar

- `update_passives` fonksiyonu henüz implemente edilmemiştir (`// yapilacak`).
- Savaş logları `div.log-box` içindeki satırlardan 6'şar grupla okunur; sayfa numarasıyla ilişkilendirilir.
- Her pet işleme süresi terminale yazdırılır; yavaş sayfaları tespit etmek için kullanılabilir.