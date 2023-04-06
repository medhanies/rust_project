// convert temp from fahrenheit to celsius

fn main(){

    let f = 6.0;
    let c = 6.0;

    print!("{f} f is {} c", converter_to_c(6.0));
    print!("{c} c is {} f", converter_to_f(6.0));

}

fn converter_to_f(c: f32) -> f32 {
    c * 1.8 + 32.0
}

fn converter_to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}