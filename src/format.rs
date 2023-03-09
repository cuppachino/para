use owo_colors::AnsiColors;

pub fn usize_success(hits: usize, outof: usize) -> AnsiColors {
    match hits {
        q if q == outof => AnsiColors::Green,
        0 => AnsiColors::Red,
        _ => AnsiColors::Yellow,
    }
}
