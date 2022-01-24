use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;

use rust_sbml::{LocalParameter, Model, UnitSIdRef};

#[derive(Debug, argh::FromArgs)]
/// Sabio SBML to TSV parameters.
pub struct Args {
    /// input SBML file from SABIO-rk.
    #[argh(positional)]
    file: PathBuf,
}

// Populate a map to go from SABIO ids in the model to the corresponding names
fn get_species_map(model: &mut Model) -> HashMap<String, String> {
    model
        .species
        .iter_mut()
        .map(|(id, spec)| (id.to_owned(), spec.name.take().unwrap_or_default()))
        .collect()
}

struct ExperimentalParameter {
    param: LocalParameter,
    ph: Option<String>,
    t: Option<String>,
}

fn map_local_to_exp(
    local_param: Vec<LocalParameter>,
    sabiork: &Option<rust_sbml::annotation::sabiork::Sabiork>,
) -> Vec<ExperimentalParameter> {
    let ph = sabiork.as_ref().map(|sabiork| {
        sabiork
            .get_ph()
            .map(|ph| format!("{}", ph))
            .unwrap_or_else(|| String::from("NaN"))
    });
    let t = sabiork.as_ref().map(|sabiork| {
        sabiork
            .get_temperature()
            .map(|t| format!("{}", t))
            .unwrap_or_else(|| String::from("NaN"))
    });
    local_param
        .into_iter()
        .map(|local_param| ExperimentalParameter {
            param: local_param,
            ph: ph.clone(),
            t: t.clone(),
        })
        .collect()
}

fn main() {
    let args: Args = argh::from_env();

    let mut file_h = std::fs::File::open(args.file).expect("File must exist!");
    let mut file_str = String::new();
    file_h
        .read_to_string(&mut file_str)
        .expect("Unable to read the file");
    let mut model = Model::parse(&file_str).unwrap();
    let species = get_species_map(&mut model);

    println!("id\tvalue\tunit\tspecies_name\tpH\tT");
    model
        .reactions
        .iter_mut()
        .filter_map(|(_id, r)| r.kinetic_law.take())
        .flat_map(|kl| {
            let sabiork = kl.annotation.unwrap().sabiork.take();
            map_local_to_exp(kl.list_of_local_parameters.local_parameter, &sabiork)
        })
        .for_each(|param| {
            println!(
                "{}\t{}\t{:?}\t{}\t{}\t{}",
                param.param.id,
                param.param.value.unwrap_or_default(),
                param
                    .param
                    .units
                    .as_ref()
                    .unwrap_or(&UnitSIdRef::CustomUnit(String::from(""))),
                species
                    .iter()
                    .find_map(|(id, name)| if param.param.id.contains(id) {
                        Some(name.as_str())
                    } else {
                        None
                    })
                    .unwrap_or(""),
                param.ph.unwrap_or_else(|| String::from("NaN")),
                param.t.unwrap_or_else(|| String::from("NaN")),
            )
        });
}
