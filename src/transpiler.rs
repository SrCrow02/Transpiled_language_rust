use regex::Regex;
use std::collections::HashMap;

pub fn transpile_code(input: &str) -> String {
    let re = Regex::new(r#"^(\w+)\s*(.*)$"#).unwrap();
    let mut output = String::new();
    let mut commands: HashMap<&str, fn(&str) -> String> = HashMap::new();
    let mut inside_function = false;
    let mut current_function = String::new();
    let mut indentation = 0;
    let mut add_actix_code = false;

    commands.insert("route", route_command);
    commands.insert("function", function_command);
    commands.insert("print", print_command);
    commands.insert("return", return_command);
    commands.insert("endfunction", endfunction_command);
    commands.insert("for", for_loop_command);
    commands.insert("endfor", endfor_command);
    commands.insert("webapp", webapp_command);
    commands.insert("while", while_command);
    commands.insert("endwhile", endwhile_command);
    commands.insert("if", if_command);
    commands.insert("endif", endif_command);
    commands.insert("else", else_command);
    commands.insert("endelse", endelse_command);

    for line in input.lines() {
        if let Some(caps) = re.captures(line.trim()) {
            let command = caps.get(1).map_or("", |m| m.as_str());
            let args = caps.get(2).map_or("", |m| m.as_str());

            if command == "function" || command == "route" {
                inside_function = true;
                current_function = args.split_whitespace().next().unwrap_or("").to_string();
                indentation = 0;  // Reset indentation for new function
            }

            if let Some(func) = commands.get(command) {
                let code = func(args);
                output.push_str(&"    ".repeat(indentation));
                output.push_str(&code);
                output.push('\n');
                
                if command == "route" || command == "function" {
                    indentation += 1;  // Increase indentation after function/route declaration
                } else if command == "endfunction" {
                    inside_function = false;
                    current_function.clear();
                    indentation = 0;
                } else if command == "webapp" {
                    add_actix_code = true;  // Set flag to add Actix code
                }
            } else if inside_function {
                output.push_str(&"    ".repeat(indentation));
                output.push_str(line.trim());
                output.push('\n');
            } else {
                output.push_str(&format!("// Unknown command or misplaced code: {}\n", line.trim()));
            }
        } else if !line.trim().is_empty() {
            if inside_function {
                output.push_str(&"    ".repeat(indentation));
                output.push_str(line.trim());
                output.push('\n');
            } else {
                output.push_str(&format!("// {}\n", line.trim()));
            }
        }
    }

    if add_actix_code {
        if !output.contains("async fn main()") {
            output.push_str("\n#[actix_web::main]\n");
            output.push_str("async fn main() -> std::io::Result<()> {\n");
            output.push_str("    HttpServer::new(|| {\n");
            output.push_str("        App::new()\n");
            output.push_str("            .service(hello)\n");
            output.push_str("    })\n");
            output.push_str("    .bind(\"127.0.0.1:8080\")?\n");
            output.push_str("    .run()\n");
            output.push_str("    .await\n");
            output.push_str("}\n");
        }
    }

    output
}

fn route_command(args: &str) -> String {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.len() >= 2 {
        format!("#[actix_web::get({})]\nasync fn {}() -> impl Responder {{", parts[0], parts[1])
    } else {
        "// Invalid route declaration".to_string()
    }
}

fn for_loop_command(args: &str) -> String {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.len() >= 3 {
        format!("for {} in {} {{", parts[0], parts[2])
    } else {
        "// Invalid for loop syntax".to_string()
    }
}

fn endfor_command(_: &str) -> String {
    "}".to_string()
}

fn while_command(args: &str) -> String {
    let conditions = args.trim();
    if !conditions.is_empty() {
        format!("while {} {{", conditions)
    } else {
        "// Invalid while loop syntax".to_string()
    }
}

fn endwhile_command(_: &str) -> String {
    "}".to_string()
}
fn function_command(args: &str) -> String {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.len() >= 1 {
        format!("fn {}() {{", parts[0])
    } else {
        "// Invalid function declaration".to_string()
    }
}

fn if_command(args: &str) -> String {
    let conditions = args.trim();
    if !conditions.is_empty() {
        format!("if {} {{", conditions)
    } else {
        "// Invalid if syntax".to_string()
    }
}

fn endif_command(_: &str) -> String {
    "}".to_string()
}

fn else_command(_: &str) -> String {
    "else {".to_string()
}

fn endelse_command(_: &str) -> String {
    "}".to_string()
}

fn print_command(message: &str) -> String {
    format!("println!({});", message)
}

fn return_command(args: &str) -> String {
    format!("{}", args)
}

fn endfunction_command(_: &str) -> String {
    "}".to_string()
}

fn webapp_command(_: &str) -> String {
    "".to_string()
}