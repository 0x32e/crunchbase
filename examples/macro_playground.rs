macro_rules! test_add {
    ($a: expr) => {
        $a       
    };

    ($a: expr, $b: expr) => {
        { $a + $b } 
    };

    ($a: expr, $($b:tt)*) => {
        {
            $a+test_add!($($b)*)
        }
    };
}

fn main() {
    let res = test_add!(1, 2, 3, 4, 5);
    println!("{}", res);
}

