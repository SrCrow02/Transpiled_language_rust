use regex::Regex;
use std::collections::HashMap;

pub fn transpile_code(input: &str) -> String {
    // Compilar a expressão regular para análise de comandos
    let re = Regex::new(r#"^(\w+)\s*(.*)$"#).unwrap();
    
    // Inicializar variáveis para controle do processo de transpilação
    let mut output = String::new();
    let mut commands: HashMap<&str, fn(&str) -> String> = HashMap::new();
    let mut inside_function = false;
    let mut inside_struct = false;
    let mut current_function = String::new();
    let mut indentation = 0;
    let mut add_actix_code = false;

    // Mapear comandos para suas respectivas funções de processamento
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
    commands.insert("let", let_command);
    commands.insert("var", mut_command);
    // commands.insert("match", match_command);
    // commands.insert("endmatch", endmatch_command);
    commands.insert("struct", struct_command);
    commands.insert("endstruct", endstruct_command);
    commands.insert("array", array_command);

    // Processar cada linha do input
    for line in input.lines() {
        if let Some(caps) = re.captures(line.trim()) {
            let command = caps.get(1).map_or("", |m| m.as_str());
            let args = caps.get(2).map_or("", |m| m.as_str());

            // Verificar se estamos entrando em uma função ou rota
            if command == "function" || command == "route" {
                inside_function = true;
                current_function = args.split_whitespace().next().unwrap_or("").to_string();
                indentation = 0;  // Reiniciar a indentação para nova função
            }

            // Verificar se estamos entrando em uma struct
            if command == "struct" {
                inside_struct = true;
            }

            // Processar o comando se ele existir no mapa de comandos
            if let Some(func) = commands.get(command) {
                let code = func(args);
                output.push_str(&"    ".repeat(indentation));
                output.push_str(&code);
                output.push('\n');
                
                // Ajustar indentação após declarações específicas
                match command {
                    "route" | "function" | "for" | "while" | "if" | "else" | "match" | "struct" => indentation += 1,
                    "endfunction" | "endfor" | "endwhile" | "endif" | "endelse" | "endmatch" | "endstruct" => {
                        indentation = indentation.saturating_sub(1);
                        if command == "endfunction" {
                            inside_function = false;
                            current_function.clear();
                        }
                        if command == "endstruct" {
                            inside_struct = false;
                        }
                    },
                    "webapp" => add_actix_code = true,
                    _ => {}
                }
            } else if inside_function || inside_struct {
                // Processar conteúdo dentro de funções ou structs
                output.push_str(&"    ".repeat(indentation));
                output.push_str(line.trim());
                if inside_struct {
                    output.push_str(",");  // Adiciona vírgula para campos da struct
                } else if inside_function && !["if", "else", "for", "while"].contains(&command) {
                    output.push_str(";");  // Adiciona ponto e vírgula dentro de funções
                }
                output.push('\n');
            } else {
                // Linhas desconhecidas ou mal posicionadas são comentadas
                output.push_str(&format!("// Unknown command or misplaced code: {}\n", line.trim()));
            }
        } else if !line.trim().is_empty() {
            // Processar linhas que não são comandos
            if inside_function || inside_struct {
                output.push_str(&"    ".repeat(indentation));
                output.push_str(line.trim());
                if inside_struct {
                    output.push_str(",");  // Adiciona vírgula para campos da struct
                } else if inside_function && !line.trim().ends_with('{') && !line.trim().ends_with('}') {
                    output.push_str(";");  // Adiciona ponto e vírgula dentro de funções
                }
                output.push('\n');
            } else {
                // Comentar linhas fora de funções ou structs
                output.push_str(&format!("// {}\n", line.trim()));
            }
        }
    }

    // Adicionar código Actix se necessário
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
    format!("println!{};", message.trim_matches('"'))
}

fn return_command(args: &str) -> String {
    format!("return {};", args)
}

fn endfunction_command(_: &str) -> String {
    "}".to_string()
}

fn webapp_command(_: &str) -> String {
    "".to_string()
}

fn let_command(args: &str) -> String {
    format!("let {};", args)
}   

fn mut_command(args: &str) -> String {
    format!("let mut {};", args)
}

// fn match_command(args: &str) -> String {
//     format!("match {} {{", args)
// }

// fn endmatch_command(_: &str) -> String {
//     "}".to_string()
// }

fn struct_command(args: &str) -> String {
    format!("struct {} {{", args)
}

fn endstruct_command(_: &str) -> String {
    "}".to_string()
}

fn array_command(args: &str) -> String {
    let parts: Vec<&str> = args.split("=").collect();
    if parts.len() == 2 {
        let var_name = parts[0].trim();
        let values = parts[1].trim();
        format!("let {} = [{}]", var_name, values)
    } else {
        "// Invalid array declaration".to_string()
    }
}