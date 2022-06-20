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
