const HEIGHT : usize = 256;
const WIDTH : usize = 256;
fn main(){

    println!("P3");    
    println!("{} {}",WIDTH,HEIGHT);
    println!("255");
    for j in (0..HEIGHT).rev() {
        eprintln!("Scanlines remaining:{}",j);
        for i in 0..WIDTH {
            let r = (i as f64) / ((HEIGHT-1) as f64);
            let g = (j as f64) / ((WIDTH-1) as f64);
            let b = 0.25;
            
            let ir : usize =  (r*255.999) as usize; 
            let ig : usize =  (g*255.999) as usize;
            let ib : usize =  (b*255.999) as usize;
            
            println!("{} {} {}",ir,ig,ib);
        }
    }
    eprintln!("Done!");
}