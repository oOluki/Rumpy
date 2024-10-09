enum SymbolTypes {
    SymbolScalar,
    SymbolVector(usize),
    SymbolMatrice(usize, usize),
}

struct Symbol{
    name: String,
    symbol_type: SymbolTypes,
    data: Option<usize>,
}

// returns the size of the symbol if true or 0 otherwise
fn is_special_char(input: &char) -> bool{
    match input {
        '[' => return true,
        ']' => return true,
        '(' => return true,
        ')' => return true,
        ',' => return true,
        '=' => return true,
        '+' => return true,
        '-' => return true,
        '*' => return true,
        '/' => return true,
        ':' => return true,
        _   => return false
    };
}

fn tokenize_further<'b, 'a>(output: &mut std::vec::Vec<&'a str>, input: &'b str) where 'b: 'a {

    let mut last_term_pos: usize = 0;
    let mut i: usize = 0;

    for character in input.chars() {

        if is_special_char(&character) {
            if last_term_pos < i {
            output.push(&input[last_term_pos..i]);
            }
            output.push(&input[i..i+1]);
            last_term_pos = i + 1;
        }

        i += 1;
    }
    if last_term_pos < i {
        output.push(&input[last_term_pos..i]);
    }
}

fn tokenize(input: &str) -> std::vec::Vec<&str> {
    let mut output: std::vec::Vec<&str> = vec![];

    for token in input.split_whitespace() {
        tokenize_further(&mut output, token);
    }

    return output;
}


fn get_type_string(input: &SymbolTypes) -> Option<String> {
    return match input {
        SymbolTypes::SymbolScalar => Some(String::from("scal")),
        SymbolTypes::SymbolVector(dimension) => Some(format!("vec[{dimension}]")),
        SymbolTypes::SymbolMatrice(lines, columns ) => Some(format!("mat[{lines}][{columns}]"))
        //_ => None
    }
}


fn find_sym<'a>(sym_str: &str, symbol_table: &'a std::vec::Vec<Symbol>) -> Option<&'a Symbol>{
    for symb in symbol_table {
        if sym_str == symb.name {
            return Some(symb);
        }
    }
    return None;
}


fn get_type(tokens: &std::vec::Vec<&str>, offset: usize) -> Option<SymbolTypes> {

    if tokens[offset] == "scal" {
        return Some(SymbolTypes::SymbolScalar);
    } else if tokens[offset] == "mat" && tokens.len() - offset == 7{
        if tokens[offset + 1] != "[" || tokens[offset + 3] != "]"
        || tokens[offset + 4] != "[" || tokens[offset + 6] != "]" {
            return None;
        }
        let lines = tokens[offset + 2].parse();
        let column = tokens[offset + 5].parse();
        if lines.is_ok() && column.is_ok() {
            return Some(SymbolTypes::SymbolMatrice(lines.expect(""), column.expect("")));
        }        
    } else if tokens[offset] == "vec" && tokens.len() - offset == 4{
        if tokens[offset + 1] != "[" || tokens[offset + 3] != "]" {
            return None;
        }
        let dim = tokens[offset + 2].parse();
        if dim.is_ok() {
            return Some(SymbolTypes::SymbolVector(dim.expect("")));
        }
    }
    return None;
}

fn parse_declaration(statement: &std::vec::Vec<&str>, offset: usize) -> Option<Symbol> {
    if statement.len() < 2 + offset || statement[offset + 1] != ":"{ 
        return None;
    }
    let name = statement[offset];
    let symb_type = get_type(statement, 2 + offset);
    
    if symb_type.is_some() {
        return Some(Symbol{name: String::from(name), symbol_type: symb_type?, data: None});
    }
    return None;
}

fn display_symbol(symbol_str: &str, symbol_table: &std::vec::Vec<Symbol>, data_buffer: &std::vec::Vec<f64>){


    let symbol = find_sym(symbol_str, symbol_table).expect("Symbol Not Found");
    

    if Option::is_some(&symbol.data) {
        println!(
            "Symbol: {symb_type} {name} = {value}",
            symb_type=get_type_string(&symbol.symbol_type).expect("Invalid Type"),
            name=symbol.name, value=data_buffer[symbol.data.expect("Expected Data")]
        );
    } else {
        println!(
            "Symbol: {name}: {symb_type}",
            symb_type=get_type_string(&symbol.symbol_type).expect("Invalid Type"),
            name=symbol.name
        );
    }

}


fn main(){

    println!("Enter Anything:");

    let mut input_buffer = String::new();

    let data_buffer: std::vec::Vec<f64> = vec![];

    let mut symbols: std::vec::Vec<Symbol> = vec![];

    let mut i: u8 = 0;

    while i < 5 {

        i+=1;

        input_buffer.clear();

        std::io::stdin().read_line(&mut input_buffer).expect("Could Not Read Line");

        input_buffer = String::from(input_buffer.strip_suffix("\n").unwrap_or(&input_buffer));

        let tokens: std::vec::Vec<&str> = tokenize(&input_buffer);

        if tokens.len() == 0 {
            continue;
        }

        if tokens[0] == "close" || tokens[0] == "exit" {
            break;
        }
        if tokens[0] == "show" {
            if tokens.len() != 2 {
                println!("Expected Symbol, Got {} Instead", input_buffer);
                continue;
            }
            display_symbol(tokens[1], &symbols, &data_buffer);
            continue;
        }
        if tokens[0] == "let" {
            let symbol = parse_declaration(&tokens, 1);
            if symbol.is_some() {
                symbols.push(symbol.expect(""));
            } else {
                println!("Invalid Symbol Declaration {}", input_buffer);
            }
            continue;
        }

        println!("Invalid Input");       
        
    }

    

}

