mod generator;
use generator::Generator;

fn main() {
    // example run / testing, showing off the generator
    let mut generator = Generator::default();
    generator.add_part("name", vec!["OZ-Forge-X3","AX-9"]);
    generator.add_part("material", vec!["glass", "rusted steel"]);
    generator.add_part("form",vec!["spherical","serpentine"]);
    generator.add_part("core", vec!["crackling static", "molten metal"]);

    generator.add_template("{name} is a {material}-bodied {form} drone filled with {core}.");
    for _ in 0..3{
        println!("{:#?}", generator.generate().unwrap() );
    }
    match generator.export_toml("./test.toml"){
        Ok(_) => println!("TOML exported succeeded"),
        Err(e) => eprintln!("TOML export failed: {}", e),
    }
    match generator.import_toml("./test.toml"){
        Ok(o) => println!("{:#?}", o.generate().unwrap() ),
        Err(e) => eprintln!("TOML import failed: {}", e)
    }
}
