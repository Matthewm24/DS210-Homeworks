fn main(){
    let mut F : [u128;181] = [0;181];
    F[1] = 1;
    for i in 0..181 {
        if i > 1{
            F[i] = F[i - 1] + F[i - 2] as u128;
        }
    }
    for i in 0..181 {
        println!("F[{}] = {}", i, F[i]);
    }

}
