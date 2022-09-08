#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        info!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),* $(,)?) => { trace!(($($xs),*)) }
}

#[macro_export]
macro_rules! rotate {
    ($shift:expr, [ $( $loc:expr ),* $(,)? ]) => {
        let mut colors = vec![ $( $loc ),* ];
        colors.rotate_right($shift);
        rotate!(@assign, colors, 0, $( $loc ),* );
    };
    (@assign, $colors:expr, $index:expr $(,)? ) => {};
    (@assign, $colors:expr, $index:expr, $loc:expr $(,)?) => {
        $loc = $colors[$index];
    };
    (@assign, $colors:expr, $index:expr, $loc:expr, $( $rest:expr ),*) => {
        $loc = $colors[$index];
        rotate!(@assign, $colors, ($index + 1), $( $rest ),*);
    };
}

#[macro_export]
macro_rules! cube {
    (
        $u00:tt $u01:tt $u02:tt ;
        $u10:tt $u11:tt $u12:tt ;
        $u20:tt $u21:tt $u22:tt ;
        $f00:tt $f01:tt $f02:tt $r00:tt $r01:tt $r02:tt $b00:tt $b01:tt $b02:tt $l00:tt $l01:tt $l02:tt ;
        $f10:tt $f11:tt $f12:tt $r10:tt $r11:tt $r12:tt $b10:tt $b11:tt $b12:tt $l10:tt $l11:tt $l12:tt ;
        $f20:tt $f21:tt $f22:tt $r20:tt $r21:tt $r22:tt $b20:tt $b21:tt $b22:tt $l20:tt $l21:tt $l22:tt ;
        $d00:tt $d01:tt $d02:tt ;
        $d10:tt $d11:tt $d12:tt ;
        $d20:tt $d21:tt $d22:tt $( ; )?
    ) => {
        Cube::new(
            cube!(@face $f00 $f01 $f02 ; $f10 $f11 $f12 ; $f20 $f21 $f22),
            cube!(@face $b00 $b01 $b02 ; $b10 $b11 $b12 ; $b20 $b21 $b22),
            cube!(@face $u00 $u01 $u02 ; $u10 $u11 $u12 ; $u20 $u21 $u22),
            cube!(@face $d00 $d01 $d02 ; $d10 $d11 $d12 ; $d20 $d21 $d22),
            cube!(@face $l00 $l01 $l02 ; $l10 $l11 $l12 ; $l20 $l21 $l22),
            cube!(@face $r00 $r01 $r02 ; $r10 $r11 $r12 ; $r20 $r21 $r22),
        )
    };

    (@face
        $c00:tt $c01:tt $c02:tt ;
        $c10:tt $c11:tt $c12:tt ;
        $c20:tt $c21:tt $c22:tt $( ; )?
    ) => {
        Face::new(
            [
                [cube!(@color $c00), cube!(@color $c01), cube!(@color $c02)],
                [cube!(@color $c10), cube!(@color $c11), cube!(@color $c12)],
                [cube!(@color $c20), cube!(@color $c21), cube!(@color $c22)],
            ]
        )
    };

    (@color W) => {
        Color::White
    };
    (@color Y) => {
        Color::Yellow
    };
    (@color R) => {
        Color::Red
    };
    (@color O) => {
        Color::Orange
    };
    (@color B) => {
        Color::Blue
    };
    (@color G) => {
        Color::Green
    };
    (@color .) => {
        Color::Other
    };
    (@color *) => {
        Color::Wildcard
    };
}
