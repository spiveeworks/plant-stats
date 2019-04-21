
pub type WaterMap = [[f64;32];32];

// proportion of water that diffuses out of a tile per day
const DIFF_RATE: f64 = 0.7;
// proportion of water that is destroyed in a tile per day
const EVAP_COEFF: f64 = 0.0;
// ratio between adjacent and diagonal water diffusion
const ADJ_PROP: f64 = 0.65;

const ADJ_COEFF: f64 = DIFF_RATE / 4.0 * ADJ_PROP;
const DIAG_COEFF: f64 = DIFF_RATE / 4.0 * (1.0 - ADJ_PROP);

const TOTAL_LOST_COEFF: f64 = 4.0 * ADJ_COEFF + 4.0 * DIAG_COEFF + EVAP_COEFF;

pub fn diffuse_water(map: &mut WaterMap) {
    let mut diff: WaterMap = [[0.0; 32]; 32];
    for i in 1..31 {
        for j in 1..31 {
            let curr = map[i][j];

            diff[i-1][j-1] += curr * DIAG_COEFF;
            diff[i][j-1] += curr * ADJ_COEFF;
            diff[i+1][j-1] += curr * DIAG_COEFF;

            diff[i-1][j] += curr * ADJ_COEFF;
            diff[i][j] += curr * (-TOTAL_LOST_COEFF);
            diff[i+1][j] += curr * ADJ_COEFF;

            diff[i-1][j+1] += curr * DIAG_COEFF;
            diff[i][j+1] += curr * ADJ_COEFF;
            diff[i+1][j+1] += curr * DIAG_COEFF;
        }
    }
    for i in 0..32 {
        diff[ i][ 0] = 0.0;
        diff[ i][ 1] = 0.0;
        diff[ i][30] = 0.0;
        diff[ i][31] = 0.0;
        diff[ 0][ i] = 0.0;
        diff[ 1][ i] = 0.0;
        diff[30][ i] = 0.0;
        diff[31][ i] = 0.0;
    }
    for i in 0..32 {
        for j in 0..32 {
            map[i][j] += diff[i][j];
        }
    }
}

pub fn rain(map: &mut WaterMap) {
    for i in 2..30 {
        for j in 2..30 {
            map[i][j] = (map[i][j] + 200.0) / 2.0;
        }
    }
}
