use scrab::hex_grid::*;
fn main() {
    println!("let cords = [");
    for i in 0..37 {
        let (x,y) = HexGrid::<()>::usize_to_cordinate(i).to_pixel(20.0);
        println!("[{x},{y}],");
    }
    println!("];");
}
