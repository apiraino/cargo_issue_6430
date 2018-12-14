mod submod;
use crate::submod::MyStruct;

#[test]
fn test_db() {
    let state = MyStruct::new();
    state.add_num();
}
