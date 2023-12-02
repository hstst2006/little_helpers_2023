pub mod listify 
{
    use std::fs::read_to_string;
    use std::path::Path;

    // Return a vector of the lines in a text file. Empty lines are None
    pub fn listify<T: std::str::FromStr>(input_file: &Path) -> std::io::Result<Vec<Option<T>>> 
    {
        let input_content = read_to_string(input_file)?;
        let output_content: Vec<Option<T>> = input_content.lines().map(|line| { if line == "" { None } else { line.parse().ok()}}).collect();

        Ok(output_content)
    }

}