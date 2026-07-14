// SovereignLoadCalc CLI
// Command-line interface for structural load calculation system

use sovereign_load_calc::{Dimensions, ElementType, LoadCalculator, LoadSet, Material, MaterialGrade, MaterialType};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let calculator = LoadCalculator::new();
    
    match args[1].as_str() {
        "dead_load" => handle_dead_load(&calculator, &args),
        "live_load" => handle_live_load(&calculator, &args),
        "wind_load" => handle_wind_load(&calculator, &args),
        "seismic_load" => handle_seismic_load(&calculator, &args),
        "beam" => handle_beam(&mut calculator, &args),
        "column" => handle_column(&mut calculator, &args),
        "compliance" => handle_compliance(&calculator, &args),
        "list" => handle_list(&calculator),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("SovereignLoadCalc CLI");
    println!();
    println!("Usage:");
    println!("  load_calc dead_load <length> <width> <height> <material_type> <grade>");
    println!("  load_calc live_load <occupancy>");
    println!("  load_calc wind_load <wind_speed> <height> <terrain_category>");
    println!("  load_calc seismic_load <zone> <importance> <soil_type> <weight>");
    println!("  load_calc beam <length> <width> <height> <material_grade> <dead_load> <live_load> <wind_load> <seismic_load>");
    println!("  load_calc column <length> <width> <height> <material_grade> <dead_load> <live_load> <wind_load> <seismic_load>");
    println!("  load_calc compliance <element_id> <code>");
    println!("  load_calc list");
    println!();
    println!("Example:");
    println!("  load_calc dead_load 5.0 0.3 0.5 concrete M25");
    println!("  load_calc live_load residential");
    println!("  load_calc wind_load 50.0 10.0 2");
    println!("  load_calc seismic_load 4 1 1 100.0");
    println!("  load_calc beam 5.0 0.3 0.5 M25 10.0 5.0 2.0 1.0");
}

fn handle_dead_load(calculator: &LoadCalculator, args: &[String]) {
    if args.len() < 7 {
        eprintln!("Error: Insufficient arguments for dead_load command");
        print_usage();
        std::process::exit(1);
    }
    
    let length: f64 = args[2].parse().expect("Invalid length");
    let width: f64 = args[3].parse().expect("Invalid width");
    let height: f64 = args[4].parse().expect("Invalid height");
    let material_type_str = &args[5];
    let grade_str = &args[6];
    
    let material_type = match material_type_str.to_lowercase().as_str() {
        "concrete" => MaterialType::Concrete,
        "steel" => MaterialType::Steel,
        _ => {
            eprintln!("Error: Invalid material type. Use 'concrete' or 'steel'");
            std::process::exit(1);
        }
    };
    
    let grade = match grade_str.to_uppercase().as_str() {
        "M15" => MaterialGrade::ConcreteM15,
        "M20" => MaterialGrade::ConcreteM20,
        "M25" => MaterialGrade::ConcreteM25,
        "M30" => MaterialGrade::ConcreteM30,
        "M35" => MaterialGrade::ConcreteM35,
        "M40" => MaterialGrade::ConcreteM40,
        "FE250" => MaterialGrade::SteelFe250,
        "FE415" => MaterialGrade::SteelFe415,
        "FE500" => MaterialGrade::SteelFe500,
        "FE550" => MaterialGrade::SteelFe550,
        _ => {
            eprintln!("Error: Invalid material grade");
            std::process::exit(1);
        }
    };
    
    let material = Material::new(material_type, grade);
    let dimensions = Dimensions::new(length, width, height, 0.0);
    
    let dead_load = calculator.calculate_dead_load(&material, &dimensions);
    
    println!("Dead Load: {:.2} kN", dead_load);
}

fn handle_live_load(calculator: &LoadCalculator, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Occupancy type required");
        print_usage();
        std::process::exit(1);
    }
    
    let occupancy = &args[2];
    let live_load = calculator.calculate_live_load(occupancy);
    
    println!("Live Load: {:.2} kN/m²", live_load);
}

fn handle_wind_load(calculator: &LoadCalculator, args: &[String]) {
    if args.len() < 5 {
        eprintln!("Error: Insufficient arguments for wind_load command");
        print_usage();
        std::process::exit(1);
    }
    
    let wind_speed: f64 = args[2].parse().expect("Invalid wind speed");
    let height: f64 = args[3].parse().expect("Invalid height");
    let terrain_category: u8 = args[4].parse().expect("Invalid terrain category");
    
    let wind_load = calculator.calculate_wind_load(wind_speed, height, terrain_category);
    
    println!("Wind Load: {:.4} kN/m²", wind_load);
}

fn handle_seismic_load(calculator: &LoadCalculator, args: &[String]) {
    if args.len() < 6 {
        eprintln!("Error: Insufficient arguments for seismic_load command");
        print_usage();
        std::process::exit(1);
    }
    
    let zone: u8 = args[2].parse().expect("Invalid zone");
    let importance: u8 = args[3].parse().expect("Invalid importance");
    let soil_type: u8 = args[4].parse().expect("Invalid soil type");
    let weight: f64 = args[5].parse().expect("Invalid weight");
    
    let seismic_load = calculator.calculate_seismic_load(zone, importance, soil_type, weight);
    
    println!("Seismic Load: {:.2} kN", seismic_load);
}

fn handle_beam(calculator: &mut LoadCalculator, args: &[String]) {
    if args.len() < 10 {
        eprintln!("Error: Insufficient arguments for beam command");
        print_usage();
        std::process::exit(1);
    }
    
    let length: f64 = args[2].parse().expect("Invalid length");
    let width: f64 = args[3].parse().expect("Invalid width");
    let height: f64 = args[4].parse().expect("Invalid height");
    let grade_str = &args[5];
    let dead_load: f64 = args[6].parse().expect("Invalid dead load");
    let live_load: f64 = args[7].parse().expect("Invalid live load");
    let wind_load: f64 = args[8].parse().expect("Invalid wind load");
    let seismic_load: f64 = args[9].parse().expect("Invalid seismic load");
    
    let grade = match grade_str.to_uppercase().as_str() {
        "M15" => MaterialGrade::ConcreteM15,
        "M20" => MaterialGrade::ConcreteM20,
        "M25" => MaterialGrade::ConcreteM25,
        "M30" => MaterialGrade::ConcreteM30,
        "M35" => MaterialGrade::ConcreteM35,
        "M40" => MaterialGrade::ConcreteM40,
        "FE250" => MaterialGrade::SteelFe250,
        "FE415" => MaterialGrade::SteelFe415,
        "FE500" => MaterialGrade::SteelFe500,
        "FE550" => MaterialGrade::SteelFe550,
        _ => {
            eprintln!("Error: Invalid material grade");
            std::process::exit(1);
        }
    };
    
    let material = Material::new(MaterialType::Concrete, grade);
    let dimensions = Dimensions::new(length, width, height, 0.0);
    let loads = LoadSet::new(dead_load, live_load, wind_load, seismic_load);
    
    let beam = sovereign_load_calc::StructuralElement::new(
        ElementType::Beam,
        dimensions,
        material,
        loads,
    );
    
    let element_id = beam.get_element_id();
    calculator.add_element(beam);
    
    println!("Beam created successfully!");
    println!("Element ID: {}", element_id);
    println!();
    println!("{}", calculator.get_element(&element_id).unwrap());
}

fn handle_column(calculator: &mut LoadCalculator, args: &[String]) {
    if args.len() < 10 {
        eprintln!("Error: Insufficient arguments for column command");
        print_usage();
        std::process::exit(1);
    }
    
    let length: f64 = args[2].parse().expect("Invalid length");
    let width: f64 = args[3].parse().expect("Invalid width");
    let height: f64 = args[4].parse().expect("Invalid height");
    let grade_str = &args[5];
    let dead_load: f64 = args[6].parse().expect("Invalid dead load");
    let live_load: f64 = args[7].parse().expect("Invalid live load");
    let wind_load: f64 = args[8].parse().expect("Invalid wind load");
    let seismic_load: f64 = args[9].parse().expect("Invalid seismic load");
    
    let grade = match grade_str.to_uppercase().as_str() {
        "M15" => MaterialGrade::ConcreteM15,
        "M20" => MaterialGrade::ConcreteM20,
        "M25" => MaterialGrade::ConcreteM25,
        "M30" => MaterialGrade::ConcreteM30,
        "M35" => MaterialGrade::ConcreteM35,
        "M40" => MaterialGrade::ConcreteM40,
        "FE250" => MaterialGrade::SteelFe250,
        "FE415" => MaterialGrade::SteelFe415,
        "FE500" => MaterialGrade::SteelFe500,
        "FE550" => MaterialGrade::SteelFe550,
        _ => {
            eprintln!("Error: Invalid material grade");
            std::process::exit(1);
        }
    };
    
    let material = Material::new(MaterialType::Concrete, grade);
    let dimensions = Dimensions::new(length, width, height, 0.0);
    let loads = LoadSet::new(dead_load, live_load, wind_load, seismic_load);
    
    let column = sovereign_load_calc::StructuralElement::new(
        ElementType::Column,
        dimensions,
        material,
        loads,
    );
    
    let element_id = column.get_element_id();
    calculator.add_element(column);
    
    println!("Column created successfully!");
    println!("Element ID: {}", element_id);
    println!();
    println!("{}", calculator.get_element(&element_id).unwrap());
}

fn handle_compliance(calculator: &LoadCalculator, args: &[String]) {
    if args.len() < 4 {
        eprintln!("Error: Element ID and code required");
        print_usage();
        std::process::exit(1);
    }
    
    let element_id = &args[2];
    let code = &args[3];
    
    let report = calculator.check_bis_compliance(element_id, code);
    
    println!("{}", report);
}

fn handle_list(calculator: &LoadCalculator) {
    let elements = calculator.list_elements();
    
    if elements.is_empty() {
        println!("No elements found.");
        return;
    }
    
    println!("Structural Elements ({}):", elements.len());
    println!();
    
    for element in elements {
        println!("ID: {}", element.get_element_id());
        println!("Type: {}", element.element_type.as_str());
        println!("Compliance: {}", element.compliance_status.as_str());
        println!();
    }
}
