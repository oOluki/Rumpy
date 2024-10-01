
enum AtomType {
    LiteralI64,
    LiteralF64
}


enum SymbolTypes {
    SymbolAtom(AtomType)
}

struct Symbol{
    name: String,
    symbol_type: SymbolTypes,
    data: Option<usize>,
}

fn get_type_string(input: &SymbolTypes) -> Option<String> {
    return match input {
        SymbolTypes::SymbolAtom(AtomType::LiteralI64) => Some(String::from("i64")),
        SymbolTypes::SymbolAtom(AtomType::LiteralF64) => Some(String::from("f64")),
        //_ => None
    }
}


fn get_type(input: &str) -> Option<SymbolTypes> {

    return match input {
        "i64" => Some(SymbolTypes::SymbolAtom(AtomType::LiteralI64)),
        "f64" => Some(SymbolTypes::SymbolAtom(AtomType::LiteralF64)),
        _ => None
    }
}

fn parse_daclaration(argv: &std::vec::Vec<&str>, data_buffer: &mut std::vec::Vec<f64>) -> Option<Symbol> {
    if argv.len() < 2 || argv.len() == 3{
        return None;
    } else if argv.len() == 2 {
        return Some(Symbol {
            name: argv[1].to_string(),
            symbol_type: get_type(argv[0]).expect("Expected Identifier"),
            data: None
        });
    }
    let value: f64 = argv[3].parse().expect("Expected Expression");
    let data_pos = data_buffer.len();
    data_buffer.push(value);
    return Some(Symbol {
        name: argv[1].to_string(),
        symbol_type: get_type(argv[0]).expect("Expected Identifier"),
        data: Some(data_pos)
    });
}


fn display_symbol(symbol_str: &str, symbol_table: &std::vec::Vec<Symbol>, data_buffer: &std::vec::Vec<f64>){


    let mut symbol_found: Option<&Symbol> = None;

    for symb in symbol_table {
        if symbol_str == symb.name {
            symbol_found = Some(symb);
            break;
        }
    }

    let symbol: &Symbol = symbol_found.expect("Symbol Not Found");

    

    if Option::is_some(&symbol.data) {
        println!(
            "Symbol: {symb_type} {name} = {value}",
            symb_type=get_type_string(&symbol.symbol_type).expect("Invalid Type"),
            name=symbol.name, value=data_buffer[symbol.data.expect("Expected Data")]
        );
    } else {
        println!(
            "Symbol: {symb_type} {name}",
            symb_type=get_type_string(&symbol.symbol_type).expect("Invalid Type"),
            name=symbol.name
        );
    }

}


fn main(){

    println!("Enter Anything:");

    let mut input_buffer = String::new();

    let mut data_buffer: std::vec::Vec<f64> = vec![];

    let mut symbols: std::vec::Vec<Symbol> = vec![];

    let mut i: u8 = 0;

    while i < 5 {

        i+=1;

        input_buffer.clear();

        std::io::stdin().read_line(&mut input_buffer).expect("Could Not Read Line");

        let tokens: std::vec::Vec<&str> = input_buffer.split_whitespace().collect();

        if tokens.len() == 0 || tokens[0] == "close" {
            break;
        }
        if tokens[0] == "show" {
            if tokens.len() == 1 {
                println!("Expected Symbol");
                std::process::exit(1);
            }
            display_symbol(tokens[1], &symbols, &data_buffer);
            continue;
        }

        let symbol: Symbol = parse_daclaration(&tokens, &mut data_buffer).expect("Invalid Expression");

        symbols.push(symbol);
        
    }

    

}

