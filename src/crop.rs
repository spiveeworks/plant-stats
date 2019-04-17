use super::water;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Crop {
    // biennial (harvest and replant each year)
    Root,
    // annual
    Bean, // specialize into clover :o
    // very short lived perennial that can adapt to summer or winter
    Gourd,
    // perennial? usable as straw, hay, and cereal
    Grass,
    // perennial?
    // would like to broaden to berry, but typical berries grow on bushes
    // not included in crop since woody plants would work better as their own
    // type
    //Grape,
}

#[derive(Clone, Copy, PartialEq)]
pub struct CropData {
    // seed data fields
    // note seed might get more fields in the future so no point combining
    pub species: Crop,
    pub richness: f64,
    pub volume: f64,

    pub growth: f64,
    pub matures: f64,
    pub health: f64,
    pub max_health: f64,
    pub thirst: f64,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SeedData {
    pub species: Crop,
    // [0, 1]
    pub richness: f64,
    // [0, 1]
    pub volume: f64,
}

pub type CropMap = [[Option<CropData>; 32]; 32];

pub fn create_crop(seed: SeedData) -> CropData {
    let SeedData { species, richness, volume } = seed;

    let quality = (richness + volume) / 2.0; // [0, 1]
    let hardiness = 1.0 - quality; // [0, 1]
    let agility = 0.5 * f64::abs(quality - 0.5);
    let (base_time, base_health, base_thirst) = match species {
        Crop::Root => (60.0, 100.0, 5.0),
        Crop::Bean => (20.0, 100.0, 5.0),
        Crop::Gourd => (40.0, 150.0, 5.0),
        Crop::Grass => (15.0, 50.0, 5.0),
    };
    let health = base_health * hardiness;
    CropData {
        species,
        richness,
        volume,
        growth: 0.0,
        // all crops tend towards 1.5x growing cycle?
        matures: base_time * (1.5 - agility),
        health,
        max_health: health,
        thirst: base_thirst * quality,
    }
}

pub fn update_crops(crops: &mut CropMap, water: &mut water::WaterMap) {
    for i in 0..32 {
        for j in 0..32 {
            let moisture = &mut water[i][j];

            if crops[i][j].is_none() {
                continue;
            }
            let crop = crops[i][j].as_mut().unwrap();

            if crop.health < 0.0 {
                continue;
            }
            let available = *moisture / 10.0;
            crop.health += available;
            crop.health -= crop.thirst;
            if available < crop.thirst {
                crop.growth += available / crop.thirst;
                // crop.wilt = true;
            } else {
                crop.growth += 1.0;
                // crop.wilt = false;
            }

            *moisture -= available;
        }
    }
}

