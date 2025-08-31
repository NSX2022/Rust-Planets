use std::io::stdin;
use rand::*;
use rand::rngs::StdRng;

static PLANET_CHARS: [char; 9] = ['#','+','%','H','@','M','Z','X', '█'];
static RING_CHARS: [char; 4] = ['░', ':', ' ', 'o'];

struct Planet {
    seed: u64
} impl Planet {
    fn new(seed: u64) -> Self{
        Self{
            seed
        }
    }
    fn display(&self){
        let mut rng = StdRng::seed_from_u64(self.seed);
        
        //denser planets are smaller
        let mut name = "Default Planet Name";
        let gravity: f32 = rng.random_range(0.0001f32..=14.0f32);
        let invert = 20.0f32 / gravity;
        let array_len: usize = (invert.ceil() as usize * 4) + 4;
        let mut char_array = vec![vec![' '; array_len]; array_len];
        
        let has_asteroids: bool = rng.random_bool(0.5);
        let has_rings: bool = rng.random_bool(0.5);
        
        //slope AKA m, y offset AKA b
        let rings_angle: (f32,i32) = (rng.random_range(-1.5f32..=1.5f32), 
                                      rng.random_range(-(invert as i32) / 2..=(invert as i32) / 2));
        
        let planet_char: char = PLANET_CHARS[rng.random_range(0..=PLANET_CHARS.len() - 1)];
        let ring_char: char = RING_CHARS[rng.random_range(0..=RING_CHARS.len() - 1)];
        
        let planet_start_idx = array_len / 2;
        let planet_radius = (invert.sqrt() * 2.0) as usize;

        for y in 0..array_len {
            for x in 0..array_len {
                let dx = x as i32 - planet_start_idx as i32;
                let dy = y as i32 - planet_start_idx as i32;
                let distance = ((dx * dx + dy * dy) as f32).sqrt();

                if distance <= planet_radius as f32 {
                    char_array[y][x] = planet_char;
                }
            }
        }

        if has_rings {
            let (slope, y_offset) = rings_angle;
            for y in 0..array_len {
                for x in 0..array_len {
                    let dx = x as i32 - planet_start_idx as i32;
                    let dy = y as i32 - planet_start_idx as i32;
                    let distance = ((dx * dx + dy * dy) as f32).sqrt();

                    let expected_y = slope * dx as f32 + y_offset as f32;
                    let actual_y = dy as f32;

                    if (expected_y - actual_y).abs() < 1.5 &&
                        distance < planet_radius as f32 + 8.0 {
                        char_array[y][x] = ring_char;
                    }
                }
            }
        }

        if has_asteroids {
            for _ in 0..rng.random_range(1..=(array_len/7)) {
                let asteroid_x = rng.random_range(0..array_len);
                let asteroid_y = rng.random_range(0..array_len);
                let dx = asteroid_x as i32 - planet_start_idx as i32;
                let dy = asteroid_y as i32 - planet_start_idx as i32;
                let distance = ((dx * dx + dy * dy) as f32).sqrt();

                if distance > planet_radius as f32 + 1.0  && char_array[asteroid_x][asteroid_y] == ' ' {
                    char_array[asteroid_y][asteroid_x] = '*';
                }
            }
        }
        
        for row in &char_array {
            for &ch in row {
                print!("{}", ch);
            }
            println!();
        }
        println!();
        println!("{}", name);
        println!("Gravity: {}g", gravity/5f32);
        println!("Radius: {} km", planet_radius * 5280)
    }
    
    //TODO decode a planet into its original seed
    fn decode(image:String) -> i32 {
        //find the planet_start_idx
        
        123456789
    }
}


fn main() {
    let mut rng = rand::rng();
    let p = Planet::new(rng.random_range(0..=9999999999999999));
    p.display();
    let mut input: String = " ".to_string();
    while(input == " "){
        stdin().read_line(&mut input).expect("Failed to read line");
    }
    std::process::exit(0);
}
