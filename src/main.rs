use rust_sbml::{Model, UnitSIdRef};

fn main() {
    let file_str = include_str!("../sabio597.xml");
    let mut model = Model::parse(file_str).unwrap();
    eprintln!("Parsed model {}", model.name.unwrap_or_default());
    eprintln!("Model units: {:?}", model.model_units);
    println!("id,value,unit");
    model
        .reactions
        .iter_mut()
        .filter_map(|(_id, r)| r.kinetic_law.take())
        .flat_map(|kl| kl.list_of_local_parameters.local_parameter)
        .for_each(|param| {
            println!(
                "{},{},{:?}",
                param.id,
                param.value.unwrap_or_default(),
                param
                    .units
                    .as_ref()
                    .unwrap_or(&UnitSIdRef::CustomUnit(String::from("")))
            )
        });
}
