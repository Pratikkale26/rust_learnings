fn main() {
    let mut v: Vec<u32> = vec![1, 2, 3];
    
    print!("before - {}", v.len());
    println!("{:?}", v);
    
    v.push(4);
    
    print!("after - {}", v.len());
    println!("{:?}", v);
}