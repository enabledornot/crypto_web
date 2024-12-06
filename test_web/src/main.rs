pub mod aks;
// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 
fn main() {
    // for i in 1..10000 {
    //     if aks::aks_test(i).result {
    //         println!("{}",i);
    //     }
    // }
    // let mut pasc = aks::generate_pascal(4,2,100);
    // println!("{:?}",pasc);
    // aks::apply_modxr(&mut pasc,100,2);
    // println!("{:?}",pasc);
    // let mut vec![1,3,3,1];

    let result = aks::aks_test(1601);
    println!("{}",result.result);
    // println!("{:?}",aks::generate_pascal(45,5,45));
    // println!("{:?}",aks::generate_pascal_fast(45,5,45));
    // let rslt = aks::aks_test(79);
    // println!("{}",rslt);
    // let rslt = aks::m_order(31,7);
    // if let Some(rslt) = aks::m_order(31,29) {
    //     println!("{}",rslt);
    // }
}
