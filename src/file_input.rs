const VALID_CHARS: [char; 3] = ['0', '1', '\n'];

pub struct FileData {
    pub data: Vec<u8>,
    pub rows: usize,
    pub cols: usize,
}

pub fn read_file(path: &str) -> Result<FileData, &'static str> {
    let file_data = std::fs::read_to_string(path);
    if file_data.is_err() {
        return Err("Could not read file.");
    }
    let file_data = file_data.unwrap();
    let file_data = file_data.replace("\r\n", "\n");
    if !file_data.ends_with('\n') {
        return Err("File must end with new line.");
    }
    validate_used_chars(&file_data)?;

    let cols = determine_cols(&file_data)?;
    let rows = determine_rows(&file_data, cols)?;

    let file_data = convert_string_data(&file_data);

    Ok(FileData {
        data: file_data,
        rows,
        cols,
    })
}

fn convert_string_data(data: &str) -> Vec<u8> {
    data.chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            _ => panic!(),
        })
        .collect()
}

fn determine_cols(file_data: &str) -> Result<usize, &'static str> {
    let cols = file_data.chars().position(|c| c == '\n').ok_or_else(|| {
        panic!("This should not happen: Allowed file not having new line at the end.")
    })?;

    Ok(cols)
}

fn determine_rows(file_data: &str, cols: usize) -> Result<usize, &'static str> {
    let has_consistent_line_length = file_data.len() % (cols + 1) == 0;
    if !has_consistent_line_length {
        return Err("All rows in the file must have same length.");
    }

    Ok(file_data.len() / (cols + 1))
}

fn validate_used_chars(file_data: &str) -> Result<(), &'static str> {
    let uses_invalid_chars = file_data.chars().any(|c| !VALID_CHARS.contains(&c));
    if uses_invalid_chars {
        return Err("File contains invalid characters (only 0s and 1s allowed).");
    }

    Ok(())
}