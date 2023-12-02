pub mod listify 
{
    use std::fs::read_to_string;
    use std::path::Path;

    // Return a vector of the lines in a text file. Empty lines are None
    pub fn listify<T: std::str::FromStr>(input_file: &Path, delimiter: char) -> std::io::Result<Vec<T>>
    {
        let input_content = read_to_string(input_file)?;
        
        let output_content: Vec<T> = input_content.split(delimiter).filter_map(|row| row.parse().ok()).collect();

        // Returns lines as Some(content), and empty lines as None
        //let output_content: Vec<Option<T>> = input_content.split(delimiter).map(|row| { if row == "" { None } else { row.parse().ok()}}).collect();

        Ok(output_content)
    }

}