pub fn compute_abv(og: f32, fg: f32) -> f32 {
    (og - fg) * 0.5
}

pub fn check_ratios(ratios: Vec<u8>) -> bool {
    ratios.iter().sum::<u8>() != 100
}

/// g/L
pub fn compute_total_extract(og: f32) -> f32 {
    (0.9974 / ((1.0 / og) - 0.00382) + 0.01) * 10.0
}

pub fn compute_per_malt_extractable(total_extract: f32, malt_ratio: u8, efficiency: u8) -> f32 {
    (total_extract * (malt_ratio as f32 / 100.0)) / (efficiency as f32 / 100.0)
}

pub fn compute_grain_bill(
    batch_size: u16,
    malt_extractable: f32,
    malt_humidity: f32,
    malt_extract: f32,
) -> f32 {
    // 81 /
    (malt_extractable / (malt_extract / 100.0)) / (1.0 - (malt_humidity / 100.0))
        * batch_size as f32
}

pub fn compute_final_gravity(og: f32, attenuation: f32) -> f32 {
    og - og * (attenuation / 100.0)
}

pub fn compute_mash_water_vol(grain_weight: f32, water_ratio: f32) -> f32 {
    grain_weight / 1000.0 * water_ratio
}

pub fn compute_post_mash_water_vol(mash_water_vol: f32, grain_weight: f32) -> f32 {
    mash_water_vol - ((grain_weight / 1000.0) * 0.8)
}

pub fn compute_sparge_water_vol(
    batch_size: u16,
    evaporation_rate: f32,
    post_mash_water_vol: f32,
) -> f32 {
    (batch_size as f32 + (batch_size as f32 * (evaporation_rate / 100.0))) - post_mash_water_vol
}

pub fn compute_pre_ebullition_water_vol(sparge_water_vol: f32, post_mash_water_vol: f32) -> f32 {
    sparge_water_vol + post_mash_water_vol
}

pub fn compute_mcu(ebc: u8, grain_weight: f32, batch_size: u16) -> f32 {
    (4.23 * (ebc as f32) * (grain_weight / 1000.0)) / batch_size as f32
}

/// Morey's formula
/// https://www.brassageamateur.com/wiki/Formules#Couleur_de_la_bi.C3.A8re
///
pub fn compute_ebc(total_mcu: f32) -> u8 {
    (2.939 * total_mcu.powf(0.6859)) as u8
}

pub fn compute_bugu(ibu: f32, og: f32) -> f32 {
    ibu / ((convert_plato_to_sg(og) - 1.0) * 1000.0)
}

/// https://www.brewersfriend.com/plato-to-sg-conversion-chart/
///
pub fn convert_plato_to_sg(plato: f32) -> f32 {
    1.0 + (plato / (258.6 - ((plato / 258.2) * 227.1)))
}

// pub fn convert_sg_to_plato(sg: f32) -> f32 {
//     (-1.0 * 616.868) + (1111.14 * sg) - (630.272 * sg.powf(2.0)) + (135.997 * sg.powf(3.0))
// }

pub fn compute_cell_count(og: f32, batch_size: u16) -> f32 {
    1_000_000.0 * (batch_size as f32 * 1000.0) * og
}

/// Hop utilization is caculated using Glenn Tinseth's formula
/// http://univers-biere.net/amertume.php
/// https://www.backtoschoolbrewing.com/blog/2016/9/5/how-to-calculate-ibus
/// https://realbeer.com/hops/research.html
///
/// density argument is expressed in plato and converted in specific gravity
pub fn compute_hop_utilization(density: f32, time: u8) -> f32 {
    let density = convert_plato_to_sg(density);

    let density_pow: f32 = density - 1.0;
    let time_pow: f32 = -0.04 * (time as f32);

    let x: f32 = 0.000125;

    1.65 * x.powf(density_pow) * ((1.0 - std::f32::consts::E.powf(time_pow)) / 4.15) + 0.10
}

pub fn compute_ibu(
    hop_utilization: f32,
    batch_size: u16,
    alpha: f32,
    hop_weight: f32,
    density: f32,
) -> f32 {
    let density_in_sg = convert_plato_to_sg(density);
    if density_in_sg > 1.050 {
        (hop_weight * hop_utilization * alpha * 10.0)
            / (batch_size as f32 * density_correction(density_in_sg))
    } else {
        (hop_weight * hop_utilization * alpha * 10.0) / batch_size as f32
    }
}

pub fn compute_hop_weight(
    hop_utilization: f32,
    batch_size: u16,
    alpha: f32,
    ibu: f32,
    density: f32,
) -> f32 {
    let density_in_sg = convert_plato_to_sg(density);
    if density_in_sg > 1.050 {
        (batch_size as f32 * density_correction(density_in_sg) * ibu)
            / (hop_utilization * alpha * 10.0)
    } else {
        (batch_size as f32 * ibu) / (hop_utilization * alpha * 10.0)
    }
}

pub fn density_correction(density: f32) -> f32 {
    1.0 + (((density / 1000.0) - 1.050) / 2.0)
}
