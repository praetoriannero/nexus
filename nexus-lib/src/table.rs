use crate::prelude::*;

pub struct Dissector {}

pub trait Dissect {
    fn new() -> Self;
    fn add_entry(&mut self);
    fn remove_entry(&mut self);
}

impl Dissect for Dissector {
    fn new() -> Self {
        Self {}
    }

    fn add_entry(&mut self) {}

    fn remove_entry(&mut self) {}
}

//
// #[macro_export]
// macro_rules! add_to_table {
//     ($eth_type:expr, $builder:ident, $table:ident) => {
//         paste! {
//             #[ctor]
//             fn [<__nexus_register_ether_type_ $builder:lower>]() {
//                 pdu_trait_assert::<$builder>();
//                 if table
//                     .write()
//                     .unwrap()
//                     .insert($eth_type, |bytes: &'_ [u8]| -> PduResult<'_> {
//                         $builder::from_bytes(bytes)
//                     })
//                     .is_some()
//                 {
//                     panic!("Ethernet types can only be added once.")
//                 };
//             }
//         }
//     };
// }
//
// #[macro_export]
// macro_rules! remove_from_table {
//     ($eth_type:expr, $builder:ident, $table:ident) => {
//         paste! {
//             #[ctor]
//             fn [<__nexus_register_ether_type_ $builder:lower>]() {
//                 pdu_trait_assert::<$builder>();
//                 if table
//                     .write()
//                     .unwrap()
//                     .insert($eth_type, |bytes: &'_ [u8]| -> PduResult<'_> {
//                         $builder::from_bytes(bytes)
//                     })
//                     .is_some()
//                 {
//                     panic!("Ethernet types can only be added once.")
//                 };
//             }
//         }
//     };
// }
