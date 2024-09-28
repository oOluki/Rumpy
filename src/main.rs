


enum AtomType {
    LiteralNone,
    LiteralI64,
    LiteralF64
}


enum SymbolTypes {
    SymbolNone,
    SymbolAtom(AtomType)
}



struct Symbol{
    name: String,
    symbol_type: SymbolTypes
}



fn get_type(input: &str) -> Option<SymbolTypes> {

    return match input {
        "i64" => Some(SymbolTypes::SymbolAtom(AtomType::LiteralI64)),
        "f64" => Some(SymbolTypes::SymbolAtom(AtomType::LiteralF64)),
        _ => None
    }
}


fn main(){

    println!("Enter Anything:");

    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).expect("Could Not Read Line");

    let tokens: std::vec::Vec<&str> = buffer.split_whitespace().collect();

    let id = tokens[0];

    match id {
        
        "i64" => println!("64 bits integer"),
        "f64" => println!("64 bits float"),

        _ => println!("Unknown Type"),
        
    }
    
    println!("Hello World")

}

