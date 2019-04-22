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
pub enum Stage {
    Seedling,
    FibreGrowth,
    FruitGrowth,
    Spreading,
    Dead,
}

#[derive(Clone, Copy, PartialEq)]
pub struct CropData {
    pub genome: SeedData,
    pub genome_derived: SeedGrowthData,

    pub stage: Stage,
    pub growth: f64,
    pub health: f64,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SeedGrowthData {
    pub seedling_time: f64,
    pub fibre_time: f64,
    pub fruit_time: f64,
    pub spread_time: f64,

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

impl Crop {
    fn derive(self: Self) -> SeedGrowthData {
        use self::Crop::*;
        match self {
            Root => SeedGrowthData {
                seedling_time: 10.0,
                fibre_time: 80.0,
                fruit_time: 80.0,
                spread_time: 80.0,
                max_health: 100.0,
                thirst: 1.0,
            },
            Bean => SeedGrowthData {
                seedling_time: 10.0,
                fibre_time: 20.0,
                fruit_time: 20.0,
                spread_time: 20.0,
                max_health: 100.0,
                thirst: 7.0,
            },
            Gourd => SeedGrowthData {
                seedling_time: 10.0,
                fibre_time: 40.0,
                fruit_time: 40.0,
                spread_time: 40.0,
                max_health: 150.0,
                thirst: 2.0,
            },
            Grass => SeedGrowthData {
                seedling_time: 10.0,
                fibre_time: 15.0,
                fruit_time: 15.0,
                spread_time: 15.0,
                max_health: 20.0,
                thirst: 8.0,
            },
        }
    }
}

impl SeedData {
    pub fn derive(self: Self) -> SeedGrowthData {
        let SeedData { richness, volume, .. } = self;
        let quality = (richness + volume) / 2.0; // [0, 1]
        let hardiness = 1.0 - quality; // [0, 1]
        let agility = 0.5 * f64::abs(quality - 0.5);

        let mut base = self.species.derive();
        base.max_health *= hardiness;
        // all crops tend towards 1.5x growing cycle?
        base.seedling_time *= 1.5 - agility;
        base.fibre_time *= 1.5 - agility;
        base.fruit_time *= 1.5 - agility;
        base.spread_time *= 1.5 - agility;
        base.thirst *= quality;

        base
    }

    pub fn crop(self: Self) -> CropData {
        let genome = self;
        let genome_derived = self.derive();
        CropData {
            genome,
            genome_derived,
            stage: Stage::Seedling,
            growth: 0.0,
            health: genome_derived.max_health,
        }
    }
}

pub type CropMap = [[Option<CropData>; 32]; 32];

/*
pub fn create_crop(seed: SeedData) -> CropData {
    CropData 
}
*/

pub fn update_crops(crops: &mut CropMap, water: &mut water::WaterMap) {
    for i in 0..32 {
        for j in 0..32 {
            let moisture = &mut water[i][j];

            if crops[i][j].is_none() {
                continue;
            }
            let mut crop_gone = false;
            let crop = crops[i][j].as_mut().unwrap();
            if crop.stage == Stage::Dead {
                crop.growth -= 2.0;
                if crop.growth < 0.0 {
                    crop_gone = true;
                }
            } else {
                let thirst = crop.genome_derived.thirst;
                let available = *moisture / 10.0;
                crop.health += available;
                crop.health -= thirst;
                if available < thirst {
                    crop.growth += available / thirst;
                    // crop.wilt = true;
                } else {
                    crop.growth += 1.0;
                    // crop.wilt = false;
                }
                if crop.health < 0.0 {
                    crop.stage = Stage::Dead;
                } else if crop.health > crop.genome_derived.max_health {
                    crop.health = crop.genome_derived.max_health;
                }
                *moisture -= available;
            }
            let mut spread = None;
            loop {
                match crop.stage {
                    Stage::Dead => break,

                    Stage::Seedling => {
                        if crop.growth > crop.genome_derived.seedling_time {
                            crop.stage = Stage::FibreGrowth;
                        } else {
                            break;
                        }
                    },
                    Stage::FibreGrowth => {
                        if crop.growth > crop.genome_derived.fibre_time {
                            crop.stage = Stage::FruitGrowth;
                        } else {
                            break;
                        }
                    },
                    Stage::FruitGrowth => {
                        if crop.growth > crop.genome_derived.fruit_time {
                            crop.stage = Stage::Spreading;
                        } else {
                            break;
                        }
                    },
                    Stage::Spreading => {
                        if crop.growth > crop.genome_derived.spread_time {
                            crop.stage = Stage::Dead;
                            spread = Some(crop.genome);
                        } else {
                            break;
                        }
                    },
                }
            }

            if let Some(genome) = spread {
                fn maybe_spread_seed(
                    crops: &mut CropMap,
                    i: usize,
                    j: usize,
                    genome: SeedData,
                ) {
                    if i >= 2 && i < 30 &&
                        j >= 2 && j < 30 &&
                        (crops[i][j].is_none() || crops[i][j].as_ref().unwrap().stage == Stage::Seedling)
                    {
                        let mut mutated = genome;
                        mutated.richness += rand_range(-0.1, 0.1);
                        if mutated.richness < 0.0 {
                            mutated.richness = 0.0;
                        } else if mutated.richness > 1.0 {
                            mutated.richness = 1.0;
                        }
                        mutated.volume += rand_range(-0.1, 0.1);
                        if mutated.volume < 0.0 {
                            mutated.volume = 0.0;
                        } else if mutated.volume > 1.0 {
                            mutated.volume = 1.0;
                        }

                        crops[i][j] = Some(mutated.crop());
                    }
                }
                maybe_spread_seed(crops, i-1, j-1, genome);
                maybe_spread_seed(crops, i, j-1, genome);
                maybe_spread_seed(crops, i+1, j-1, genome);
                maybe_spread_seed(crops, i-1, j, genome);
                maybe_spread_seed(crops, i+1, j, genome);
                maybe_spread_seed(crops, i-1, j+1, genome);
                maybe_spread_seed(crops, i, j+1, genome);
                maybe_spread_seed(crops, i+1, j+1, genome);
            }

            if crop_gone {
                crops[i][j] = None;
            }
        }
    }
}

mod rand {
    static mut DATA: [u64; 4] = [32767, 32767, 32767, 32767];
    pub fn rand_int() -> u64 {
        // safe because single threaded
        // if not then make this thread local storage
        unsafe {
            let old_root = DATA[0];
            let mut new_root = old_root;
            for i in 1..4 {
                new_root ^= DATA[i] << (2*i);
                new_root ^= DATA[i] >> (i - 1);
            }
            DATA = [DATA[1], DATA[2], DATA[3], new_root];
            old_root
        }
    }

    pub fn rand_proportion() -> f64 {
        let val = rand_int();
        (val % 65536) as f64 / 65536.0
    }

    pub fn rand_range(min: f64, max: f64) -> f64 {
        rand_proportion() * (max - min) + min
    }
}

use self::rand::*;
