use regex::Regex;

fn create_props(svg: String, component_type: &str, is_size: bool, is_color: bool) -> String {
    let mut svg_text = svg;
    let mut props_header = String::new();

    if !is_size && !is_color {
        return svg_text;
    }

    let width_pattern = Regex::new(r#"width="(\d+)"#).unwrap();
    let height_pattern = Regex::new(r#"height="(\d+)"#).unwrap();
    let fill_pattern = Regex::new(r#"fill="(\w+)"#).unwrap();
    if is_size {
        svg_text = width_pattern
            .replace_all(&svg_text, "width=\"{size}")
            .to_string();
        svg_text = height_pattern
            .replace_all(&svg_text, "height=\"{size}")
            .to_string();
    }
    if is_color {
        svg_text = fill_pattern
            .replace_all(&svg_text, "fill=\"{color}")
            .to_string();
    }

    match component_type {
        "svelte" => {
            props_header = "<script lang=\"ts\">\n".to_string();
            if is_size {
                props_header.push_str("\texport let size: number;\n");
            }
            if is_color {
                props_header.push_str("\texport let color: string;\n");
            }
            props_header.push_str("</script>\n\n");
        }
        "astro" => {
            let mut props = vec![];
            if is_size {
                props.push("size");
            }
            if is_color {
                props.push("color");
            }

            if !props.is_empty() {
                props_header = "---\nexport interface Props {\n".to_string();
                for prop in &props {
                    match prop {
                        &"size" => props_header.push_str("\tsize: number;\n"),
                        &"color" => props_header.push_str("\tcolor: string;\n"),
                        _ => {}
                    }
                }
                props_header.push_str("}\nconst { ");
                props_header.push_str(&props.join(", "));
                props_header.push_str(" }: Props = Astro.props;\n---\n\n");
            }
        }
        _ => {}
    }

    format!("{}{}", props_header, svg_text)
}

pub fn create(
    icon_name: &str,
    type_: &str,
    output_path: Option<String>,
    is_color: bool,
    is_size: bool,
) {
    println!("{}", icon_name);
    println!("{}", type_);
    println!("{:?}", output_path);
    println!("{}", is_color);
    println!("{}", is_size);

    println!(
        "{}",
        create_props(
            "<svg width=\"24\" height=\"24\" fill=\"currentColor\"/></svg>".to_string(),
            type_,
            is_size,
            is_color,
        )
    );
}
