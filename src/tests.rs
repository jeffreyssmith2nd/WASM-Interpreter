#[cfg(test)]
mod test {
    #[test]
    fn square_test() {
        let unoptimized_square_code = "
        (func $square (export \"square\") (param $p0 i32) (result i32)
        (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32)
        get_global $g0
        set_local $l0
        i32.const 16
        set_local $l1
        get_local $l0
        get_local $l1
        i32.sub
        set_local $l2
        get_local $l2
        get_local $p0
        i32.store offset=12
        get_local $l2
        i32.load offset=12
        set_local $l3
        get_local $l2
        i32.load offset=12
        set_local $l4
        get_local $l3
        get_local $l4
        i32.mul
        set_local $l5
        get_local $l5
        return)
        "

        let optimized_square_code = "
        (func $square (export \"square\") (param $p0 i32) (result i32)
        get_local $l0
        get_local $l0
        i32.mul
        return)
        "

        unimplemented();
    }
}
