fn main() {
    let mut x = 5;
    { //creating a scope 
        let y = &mut x; 
        *y += 1; 
    }
    { //creating another scope
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}