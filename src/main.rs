use caldera::{GenerationError, MapConfig, MapGenerator, MapSize};

fn main() {
    let mut map_generator = MapGenerator::new(MapSize::Medium, MapConfig::TestEdge);
    println!("{}", map_generator);
    match map_generator.wave_function_collapse() {
        Ok(()) => match map_generator.write_mv_import() {
            Ok(()) => {
                println!("Done!");
                // for cell in map_generator.cells() {
                //     println!("{}", cell);
                // }
            }
            Err(e) => println!("Failed to write mv_import.txt: {}!", e),
        },
        Err(GenerationError::InitialWeightsError) => println!("All initial weights are zero!"),
        Err(GenerationError::Contradiction) => println!("Weights went to zero during propagation!"),
    }
}
