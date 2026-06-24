use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use library::{BinaryData, FileType, SecurityInfo};

pub fn draw_bindata(data: &BinaryData) {
    // for compiler and format -------------------------------------
    let format = match data.format {
        FileType::PE => "PE",
        FileType::ELF => "ELF",
        _ => "Unkhown",
    };

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Compiler", "Format"])
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .add_row(vec![&data.compiler, &format.to_string()]);
    println!("{table}");

    // for libs -------------------------------------
    table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Libraries", "Version"])
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    for s in &data.libs {
        let splited: Vec<&str> = s.split_whitespace().collect();
        table.add_row(vec![splited[0], splited[1]]);
    }
    println!("{table}");

    // for imports -------------------------------------
    table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Import", "functions"])
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    for s in &data.imports {
        // if s.functions.is_empty() {
        //     continue;
        // }
        table.add_row(vec![&s.libraries, &s.functions.join(", ")]);
    }
    println!("{table}");

    // for security -------------------------------------
    table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Security Feature", "Availability"])
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    match &data.security {
        SecurityInfo::Pe(pe) => {
            table.add_row(vec![
                "ASLR",
                if pe.aslr {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            table.add_row(vec![
                "DEP",
                if pe.dep {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            table.add_row(vec![
                "CFG",
                if pe.cfg {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            table.add_row(vec![
                "No SEH",
                if pe.no_seh {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            table.add_row(vec![
                "High Entropy ASLR",
                if pe.hi_aslr {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
        }
        SecurityInfo::Elf(elf) => {
            table.add_row(vec![
                "PIE",
                if elf.pie {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            table.add_row(vec![
                "NX",
                if elf.nx {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            table.add_row(vec![
                "RELRO",
                if elf.relro {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            table.add_row(vec![
                "Stack Canary",
                if elf.canary {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            table.add_row(vec![
                "FORTIFY",
                if elf.fortify {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
        }
    }
    println!("{table}");
}
