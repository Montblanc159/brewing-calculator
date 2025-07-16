/// Computes alcohol in beverage
/// expressed in %/vol
///
pub fn compute_abv(og: f32, fg: f32) -> f32 {
    (og - fg) * 0.5
}

/// Ratios must add to 100
///
pub fn check_ratios(ratios: Vec<u8>) -> bool {
    ratios.iter().sum::<u8>() != 100
}

/// Expressed in g/L of sugar extract
///
pub fn compute_total_extract(og: f32) -> f32 {
    (0.9974 / ((1.0 / og) - 0.00382) + 0.01) * 10.0
}

/// Takes into account apparel's efficiency
/// Returns soluble extract in malt expressed in g/L
///
pub fn compute_per_malt_extractable(total_extract: f32, malt_ratio: u8, efficiency: u8) -> f32 {
    (total_extract * (malt_ratio as f32 / 100.0)) / (efficiency as f32 / 100.0)
}

/// Returns weight of specific malts in g
///
pub fn compute_grain_bill(
    batch_size: u16,
    malt_extractable: f32,
    malt_humidity: f32,
    malt_extract: f32,
) -> f32 {
    (malt_extractable / (malt_extract / 100.0)) / (1.0 - (malt_humidity / 100.0))
        * batch_size as f32
}

/// Returns final gravity in °P
///
pub fn compute_final_gravity(og: f32, attenuation: f32) -> f32 {
    og - og * (attenuation / 100.0)
}

/// Returns start mash water volume in L
///
pub fn compute_mash_water_vol(grain_weight: f32, water_ratio: f32) -> f32 {
    grain_weight / 1000.0 * water_ratio
}

/// Returns water volume in L after the mashing process
/// Taking into account grain retention (*0.8* - 1.1 grain weight)
///
pub fn compute_post_mash_water_vol(mash_water_vol: f32, grain_weight: f32) -> f32 {
    mash_water_vol - ((grain_weight / 1000.0) * 0.8)
}

/// Returns sparge water volume in L
/// Takes into account the evaporation rate
///
pub fn compute_sparge_water_vol(
    batch_size: u16,
    evaporation_rate: f32,
    post_mash_water_vol: f32,
) -> f32 {
    (batch_size as f32 + (batch_size as f32 * (evaporation_rate / 100.0))) - post_mash_water_vol
}

/// Returns pre ebullition water volumen in L
///
pub fn compute_pre_ebullition_water_vol(sparge_water_vol: f32, post_mash_water_vol: f32) -> f32 {
    sparge_water_vol + post_mash_water_vol
}

/// Malt's color contribution
///
pub fn compute_mcu(ebc: u8, grain_weight: f32, batch_size: u16) -> f32 {
    (4.23 * (ebc as f32) * (grain_weight / 1000.0)) / batch_size as f32
}

/// Morey's formula using MCUs to compute beer color
/// https://www.brassageamateur.com/wiki/Formules#Couleur_de_la_bi.C3.A8re
///
pub fn compute_ebc(total_mcu: f32) -> u8 {
    (2.939 * total_mcu.powf(0.6859)) as u8
}

/// https://www.brassageamateur.com/wiki/Unit%C3%A9_de_couleur_(EBC,_%C2%B0L,_SRM)
///
pub fn convert_ebc_to_srm(ebc: u8) -> f32 {
    ebc as f32 * 0.508
}

/// https://www.brassageamateur.com/wiki/Unit%C3%A9_de_couleur_(EBC,_%C2%B0L,_SRM)
///
pub fn convert_srm_to_ebc(srm: f32) -> f32 {
    srm * 1.87
}

// /// https://www.brassageamateur.com/wiki/Unit%C3%A9_de_couleur_(EBC,_%C2%B0L,_SRM)
// ///
// pub fn convert_ebc_to_lovibond(ebc: u8) -> f32 {
//     (0.375 * ebc as f32) + 0.46
// }

// /// https://www.brassageamateur.com/wiki/Unit%C3%A9_de_couleur_(EBC,_%C2%B0L,_SRM)
// ///
// pub fn convert_lovibond_to_ebc(lovibond: f32) -> f32 {
//     (lovibond - 0.46) / 0.375
// }

/// Subjective bitterness formula
///
pub fn compute_bugu(ibu: f32, og: f32) -> f32 {
    ibu / ((convert_plato_to_sg(og) - 1.0) * 1000.0)
}

/// Converts °P to SG
/// https://www.brewersfriend.com/plato-to-sg-conversion-chart/
///
pub fn convert_plato_to_sg(plato: f32) -> f32 {
    1.0 + (plato / (258.6 - ((plato / 258.2) * 227.1)))
}

/// Converts SG to °P
/// https://www.brewersfriend.com/plato-to-sg-conversion-chart/
///
pub fn convert_sg_to_plato(sg: f32) -> f32 {
    (-1.0 * 616.868) + (1111.14 * sg) - (630.272 * sg.powf(2.0)) + (135.997 * sg.powf(3.0))
}

/// Computes total cell count required for good fermentation start
///
pub fn compute_cell_count(og: f32, batch_size: u16) -> f32 {
    1_000_000.0 * (batch_size as f32 * 1000.0) * og
}

/// Hop utilization is caculated using Glenn Tinseth's formula
/// http://univers-biere.net/amertume.php
/// https://www.backtoschoolbrewing.com/blog/2016/9/5/how-to-calculate-ibus
/// https://realbeer.com/hops/research.html
///
/// `density` argument is expressed in plato and converted to specific gravity
///
pub fn compute_hop_utilization(density: f32, time: u8) -> f32 {
    let density = convert_plato_to_sg(density);

    (1.65 * 0.000125_f32.powf(density - 1.0))
        * ((1.0 - std::f32::consts::E.powf(-0.04 * (time as f32))) / 4.15)
        + 0.12
}

/// `temp` is in celsius, converted to kelvin in formula
/// https://beersmith.com/blog/2019/12/18/hop-utilization-in-the-whirlpool-for-beer-brewing/
///
pub fn compute_hop_temp_utilization(temp: f32) -> f32 {
    2.39 * (10.0_f32.powf(11.0)) * std::f32::consts::E.powf(-9773.0 / (temp + 273.15))
}

/// Returns IBUs for a specific hop addition
/// `batch_size`: liters
/// `alpha`: % (ex: 6.0)
/// `hop_weight`: g
/// `density`: °P
/// `addition_temp`: °C
///
pub fn compute_ibu(
    hop_utilization: f32,
    batch_size: u16,
    alpha: f32,
    hop_weight: f32,
    density: f32,
    addition_temp: f32,
) -> f32 {
    let alpha = alpha / 100.0;
    let density_in_sg = convert_plato_to_sg(density);

    if density_in_sg > 1.050 {
        (hop_weight
            * hop_utilization
            * compute_hop_temp_utilization(addition_temp)
            * alpha
            * 1000.0)
            / (batch_size as f32 * density_correction(density_in_sg))
    } else {
        (hop_weight
            * hop_utilization
            * compute_hop_temp_utilization(addition_temp)
            * alpha
            * 1000.0)
            / batch_size as f32
    }
}

/// Returns weight in g for a specific hop addition
/// `batch_size`: liters
/// `alpha`: % (ex: 6.0)
/// `density`: °P
/// `addition_temp`: °C
///
pub fn compute_hop_weight(
    hop_utilization: f32,
    batch_size: u16,
    alpha: f32,
    ibu: f32,
    density: f32,
    addition_temp: f32,
) -> f32 {
    let alpha = alpha / 100.0;
    let density_in_sg = convert_plato_to_sg(density);

    if density_in_sg > 1.050 {
        (batch_size as f32 * density_correction(density_in_sg) * ibu)
            / (hop_utilization * alpha * 1000.0 * compute_hop_temp_utilization(addition_temp))
    } else {
        (batch_size as f32 * ibu)
            / (hop_utilization * alpha * 1000.0 * compute_hop_temp_utilization(addition_temp))
    }
}

/// Used in hop ibu/weight estimation to counter-act
/// unaccurate values at above 1.050 densities
///
pub fn density_correction(density: f32) -> f32 {
    1.0 + (((density / 1000.0) - 1.050) / 2.0)
}

// /// CO2 pressure equilibrium
// ///
// pub fn compute_equilibrium_pressure(beer_temperature: f32, saturation_target: f32) -> f32 {
//     0.000260165 * beer_temperature.powf(2.)
//         + beer_temperature * (0.0109218 * saturation_target + 0.00799664)
//         - 0.0012163 * (-278.507) * (saturation_target - 3.22065)
// }
