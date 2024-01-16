// macro_rules! my_macro {
//     ($($field:ident : $value:expr),*, $msg:expr $(, $part:expr)* $(,)?) => {
//         println!("Options:");
//         $(
//             println!("  - {}: {}", stringify!($field), $value);
//         )*
//         println!("Message:");
//         println!("  {}", format!($msg, $($part),*));
//         println!();
//     };
// }
macro_rules! my_macro {( $($input:tt)* ) => (
    my_helper! {
        [options: ]
        $($input)*
    }
)}

macro_rules! my_helper {
    (
        [options: $($options:tt)*]
        $field:ident : $value:expr,
        $($rest:tt)*
    ) => (my_helper! {
        [options: $($options)* $field $value]
        $($rest)*
    });
    // If we reach this rule, we *know* the start of the input is not
    // a `$:ident : $:expr`
    (
        [options: $( $field:tt $value:tt )*]
        $msg:expr $(,
        $part:expr )* $(,)?
    ) => (
        /* We've finished munching the options: time to output: */
        println!("Options:");
        $(
            println!("  - {}: {}", stringify!($field), $value);
        )*
        println!("Message:");
        println!("  {}", format!($msg, $($part),*));
        println!();
    );
}

pub fn test() {
    // This is ok
    // my_macro!(foo: "bar", "Hello!");
    // my_macro!(foo: "bar", "Hello {}!", "world");
    // my_macro!(foo: "bar", "Hello {} {}!", "fellow", "rusticians");

    // This is not
    my_macro!(foo: "bar", bu: "baz", "Hello!");
    my_macro!(foo: "bar", bu: "baz", "Hello {}!", "world");
    my_macro!(foo: "bar", bu: "baz", "Hello {} {}!", "fellow", "rusticians");
}
