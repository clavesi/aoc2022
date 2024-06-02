use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    let input_replace = input.replace("\r\n", "\n");
    let groups = input_replace.split("\n\n");

    let mut max = 0;
    for group in groups {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>()?;
            sum += value;
        }
        println!("GROUP END WITH {sum}");

        if sum > max {
            max = sum;
        }
    }
    println!("MAX: {max}");

    Ok(())
}

fn read_input() -> color_eyre::Result<String> {
    let input = std::fs::read_to_string("src/input.txt").wrap_err("reading src/input.txt")?;
    return Ok(input);
}
