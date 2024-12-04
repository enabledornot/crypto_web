pub mod aks;
fn main() {
    let rslt = aks::aks_test(31);
    println!("{}",rslt);
    // let rslt = aks::m_order(31,7);
    // if let Some(rslt) = aks::m_order(31,29) {
    //     println!("{}",rslt);
    // }
}
