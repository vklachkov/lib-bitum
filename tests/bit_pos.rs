use bitum::bit_pos::BitPosition;

#[test]
fn substract() {
    for i in 0..25 {        
        let initial_pos = BitPosition::new(3, 7);
        let sub_pos = initial_pos.sub_bits(i);
        println!("Result of sub = {sub_pos:?}");
        let add_pos = sub_pos.inc_bits(i);
        assert_eq!(initial_pos, add_pos);
    }
}