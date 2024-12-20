// use std::vec;
#[derive(Debug)]
struct MyVec<T> {
    data: Vec<T>,
}

// trait AddVec<T> {
//     fn add(&self, other: &Self) -> Self;
// }

// destructuring at map
// impl<T> std::ops::Add for MyVec<T>
// where
//     T: std::ops::Add<Output = T> + Copy,
// {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         let result_data = self
//             .data
//             .iter()
//             .zip(other.data.iter())
//             .map(|(&val1, &val2)| val1 + val2)
//             .collect();
//         Self { data: result_data }
//     }
// }

// explicit dereferencing at map
// impl<T> std::ops::Add for MyVec<T>
// where
//     T: std::ops::Add<Output = T> + Copy,
// {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         let result_data = self
//             .data
//             .iter()
//             .zip(other.data.iter())
//             .map(|(val1, val2)| *val1 + *val2)
//             .collect();
//         Self { data: result_data }
//     }
// }
// this will not move the data or ownership isnt transferred.
impl<T> std::ops::Add for &MyVec<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = MyVec<T>;
    fn add(self, other: Self) -> MyVec<T> {
        let result_data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(val1, val2)| *val1 + *val2)
            .collect();
        MyVec { data: result_data }
    }
}

fn main() {
    let u8_vec_1 = MyVec {
        data: vec![10u8, 34, 56],
    };
    let u8_vec_2 = MyVec {
        data: vec![10u8, 34, 56],
    };
    let result = &u8_vec_1 + &u8_vec_2;
    println!("{:?}", result);
    println!("{:?}", u8_vec_1);
    println!("{:?}", u8_vec_2);
    // let str_1 = MyVec {
    //     data: vec!["10u8", "34", "56"],
    // };
    // let str_2 = MyVec {
    //     data: vec!["10u8", "34", "56"],
    // };
    // let result_str = str_1 + str_2;
    // println!("{:?}", result_str);
}
