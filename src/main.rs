fn main() {
    let v = 10_f32 ;

    let v2 = unsafe {
        std::mem::transmute::<_, u32>(v)
    } ;

    println!("v: {v}, v2: {v2}") ;  // Out: v: 10, v2: 1092616192
}
