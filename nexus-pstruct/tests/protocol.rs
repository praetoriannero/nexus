use nexus_pstruct::Protocol;

#[derive(Protocol, Default)]
struct ProtoTest {
    #[field]
    test: u8,
}

#[test]
fn integ() {
    let p = ProtoTest::default();
    p.marked_fields();
}
