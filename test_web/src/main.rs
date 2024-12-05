pub mod aks;
// 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 
fn main() {
    for i in 1..100 {
        if aks::aks_test(i) == 0 {
            println!("{}",i);
        }
    }
    // println!("{}",aks::aks_test(2));
    // println!("{}",aks::aks_test(3));
    // let rslt = aks::aks_test(79);
    // println!("{}",rslt);
    // let rslt = aks::m_order(31,7);
    // if let Some(rslt) = aks::m_order(31,29) {
    //     println!("{}",rslt);
    // }
}
