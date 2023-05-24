use std::env;
use std::fs;
include!("connection.rs");
include!("structs.rs");

fn main(){
    let address: String = String::from("sdf.org:70");
    let path: String = String::from("/sdf/classes/");
    if let Ok(parsed_page) = get_parsed_page(address, path){
	for gline in parsed_page{
	    println!("{}", gline.label);
	}
    } else {
	println!("Error connecting");
    }
}

fn get_parsed_page(address: String, path: String) -> Result<Vec<GopherLine>, String>{
    if let Ok(raw_page) = get_page(&address, &path) {
	let parsed_page: Vec<GopherLine> = parse_page(combine_to_lines(raw_page));
	Ok(parsed_page)
    } else {
	// TODO: Make a connection error page
	Err("Error connecting.".to_string())
    }
}

fn parse_page(lines: Vec<String>) -> Vec<GopherLine>{
    let mut parsed_page: Vec<GopherLine> = Vec::new();


    for line in lines {
	// If blankline
	if line.len() == 0 {
	    parsed_page.push(
		rawtext_parse(&"".to_string())
	    );
	    continue;
	}

	let parsed_line: GopherLine;
	let l_type = map_types(&line[0..1]);

	// Determine which parser to use based on the linetype
	match l_type {
	    LineType::Comment | LineType::EOF => continue,
	    _=> parsed_line = default_parse(&line, l_type)
	}
	parsed_page.push(parsed_line);
    }

    return parsed_page
}

fn default_parse(line: &String, l_type: LineType) -> GopherLine{
    let mut tab_indices: Vec<usize> = Vec::new();
    // Add check here to make sure len of tab_indices == 2
    for (i, c)  in line.chars().enumerate(){
	if c == '\t'{
	    tab_indices.push(i);
	}
    }

    if tab_indices.len() != 3 {
	return rawtext_parse(line);
    }

    let label_slice = &line[1..(tab_indices[0])];
    let path_slice = &line[(tab_indices[0] + 1)..(tab_indices[1])];
    let target_slice = &line[(tab_indices[1] + 1)..tab_indices[2]];
    let port_slice = &line[(tab_indices[2] + 1)..];

    return GopherLine{
	l_type: l_type,
	label: String::from(label_slice),
	path: Some(String::from(path_slice)),
	hostname: Some(String::from(target_slice)),
	port: Some(String::from(port_slice))
    }
}

fn rawtext_parse(line: &String) -> GopherLine{
    return GopherLine{
	l_type: LineType::RawText,
	label: line.to_string(),
	path: None,
	hostname: None,
	port: None
    }
}


fn combine_to_lines(vec:Vec<char>) -> Vec<String>{
    let mut line: String = String::new();
    let mut lines: Vec<String> = Vec::new();

    for c in vec {
	match c {
	    '\n' => {
		lines.push(line);
		line = String::new();
	    },
	    _ => line.push(c),
	}
    }
    return lines;
}
