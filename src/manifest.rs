let dependencies = doc
    .entry("dependencies")
    .or_insert_with(|| toml_edit::Item::Table(toml_edit::Table::new())); 