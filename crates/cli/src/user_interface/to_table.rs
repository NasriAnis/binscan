use comfy_table::Table;
use library::{BinaryData, FileType, SecurityInfo};

pub fn draw_bindata(data: &BinaryData) {
    // for compiler and format
    let format = match data.format {
        FileType::PE => "PE",
        FileType::ELF => "ELF",
        _ => "Unkhown",
    };

    let mut cmp_format_table = Table::new();
    cmp_format_table
        .set_header(vec!["Compiler", "Format"])
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
        .add_row(vec![&data.compiler, format]);

    // for libs
    let mut lib_table = Table::new();
    lib_table
        .set_header(vec!["Libraries"])
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);

    for s in &data.libs {
        lib_table.add_row(vec![&s]);
    }

    // for imports
    let mut import_table = Table::new();
    import_table
        .set_header(vec!["Import", "functions"])
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);

    for s in &data.imports {
        if s.functions.is_empty() {
            continue;
        }
        import_table.add_row(vec![&s.libraries, &s.functions.join(", ")]);
    }

    // for security
    let mut sec_table = Table::new();
    sec_table
        .set_header(vec!["Security Feature", "Availability"])
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);

    match &data.security {
        SecurityInfo::Pe(pe) => {
            sec_table.add_row(vec![
                "ASLR",
                if pe.aslr {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            sec_table.add_row(vec![
                "DEP",
                if pe.dep {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            sec_table.add_row(vec![
                "CFG",
                if pe.cfg {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            sec_table.add_row(vec![
                "No SEH",
                if pe.no_seh {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            sec_table.add_row(vec![
                "High Entropy ASLR",
                if pe.hi_aslr {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
        }
        SecurityInfo::Elf(elf) => {
            sec_table.add_row(vec![
                "PIE",
                if elf.pie {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            sec_table.add_row(vec![
                "NX",
                if elf.nx {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            sec_table.add_row(vec![
                "RELRO",
                if elf.relro {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            sec_table.add_row(vec![
                "Stack Canary",
                if elf.canary {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
            sec_table.add_row(vec![
                "FORTIFY",
                if elf.fortify {
                    "✓ Enabled"
                } else {
                    "✗ Disabled"
                },
            ]);
        }
    }

    println!("{cmp_format_table} {sec_table}");
    println!("{import_table}");
    println!("{lib_table}");
}
