use crate::prelude::*;
use std::hash::Hash;

pub type DissectionTable<T> = LazyLock<RwLock<HashMap<T, PduBuilder>>>;

pub const fn create_table<K>() -> DissectionTable<K> {
    LazyLock::new(|| RwLock::new(HashMap::new()))
}

pub trait Dissect<T>
where
    T: Hash + Eq + PartialEq,
{
    fn add<U: for<'a> Pdu<'a>>(&self, value: T);

    fn remove(&self, value: T);
}

pub fn build_from_table<'a, T>(
    dissect_table: &DissectionTable<T>,
    value: T,
    bytes: &'a [u8],
) -> Pob<'a>
where
    T: Hash + Eq + PartialEq,
{
    let Ok(table) = dissect_table.read() else {
        panic!("Failed to secure dissection table.")
    };

    if let Some(builder) = table.get(&value) {
        println!("table dissect {}", bytes.len());
        builder(bytes).ok()
    } else {
        Raw::from_bytes(bytes).ok()
    }
}

impl<T> Dissect<T> for DissectionTable<T>
where
    T: Hash + Eq + PartialEq,
{
    fn add<U: for<'a> Pdu<'a>>(&self, value: T) {
        let Ok(mut table) = self.write() else {
            panic!("Failed to secure dissection table.")
        };

        if table
            .insert(value, |bytes: &'_ [u8]| -> PduResult<'_> {
                U::from_bytes(bytes)
            })
            .is_some()
        {
            panic!("Pdu types can only be added to tables once.")
        };
    }

    fn remove(&self, value: T) {
        let Ok(mut table) = self.write() else {
            panic!("Failed to secure dissection table.")
        };

        table.remove(&value);
    }
}

#[macro_export]
macro_rules! register_pdu {
    ($value_type:expr, $builder:ident, $table:ident) => {
        paste! {
            #[ctor]
            fn [<__nexus_register_ $table:lower _ $builder:lower>]() {
                pdu_trait_assert::<$builder>();
                let Ok(mut d_table) = $table.write() else {
                    panic!("Failed to secure dissection table.")
                };

                if d_table
                    .insert($value_type, |bytes: &'_ [u8]| -> PduResult<'_> {
                        $builder::from_bytes(bytes)
                    })
                    .is_some()
                {
                    panic!("PDU types can only be added to tables once.")
                };
            }
        }
    };
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_dissection_table_create() {
        use crate::raw::Raw;
        static TEST_TABLE: DissectionTable<u8> = create_table();
        register_pdu!(0, Raw, TEST_TABLE);
    }
}
