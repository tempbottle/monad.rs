#![feature(macro_rules)]

#[macro_export]
macro_rules! mdo {
    (
        let $p: path = $e: expr ; $( $t: tt )*
    ) => {
            { let $p = $e ; mdo! { $( $t )* } }
    };
    (
        let $p: path : $ty: ty = $e: expr ; $( $t: tt )*
    ) => {
            { let $p: $ty = $e ; mdo! { $( $t )* } }
    };
    (
        $p: pat : $ty: ty <- $e: expr ; $( $t: tt )*
    ) => {
            bind($e, move |: $p : $ty | mdo! { $( $t )* } )
    };
    (
        $p: pat <- $e: expr ; $( $t: tt )*
    ) => {
            bind($e, move |: $p | mdo! { $( $t )* } )
    };
    (
        seq $e: expr ; $( $t: tt )*
    ) => {
            bind($e, move |: _ | mdo! { $( $t )* });
    };
    (
        end $e: expr
    ) => {
            $e
    };
}
