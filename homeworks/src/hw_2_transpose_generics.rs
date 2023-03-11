use std::convert::AsRef;
use std::fmt::Debug;

fn transpose<T,Line,Matrix>(matrix:Matrix) ->Vec<Vec<T>>
where 
    T:Copy + Debug,
    Line : AsRef<[T]>,
    Matrix : AsRef<[Line]>
    {
        let row_count = matrix.as_ref().len();
        let column_count = matrix.as_ref()[0].as_ref().len();

        let mut result = Vec::new();

        for row in 0..column_count
        {
            let mut each_column:Vec<T> = Vec::new();
            for col in 0..row_count
            {
                each_column.push(matrix.as_ref()[col].as_ref()[row]);
            }
            result.push(each_column);
        }
        return result;
    }

fn pretty_print<T,Line,Matrix>(matrix:Matrix) ->()
where
    T:Copy + Debug,
    Line : AsRef<[T]>,
    Matrix : AsRef<[Line]>
    {
        for row in matrix.as_ref()
        {
            println!("{:?}",row.as_ref());
        }
    }

fn main()
{
    // &[&[i32]]
    let matrix_one = transpose(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9],&[10,11,12]]);
    pretty_print(&matrix_one);
    println!("");
    // [[&str; 2]; 2]
    let matrix_two = transpose([["a", "b"], ["c", "d"],["e","f"],["g","h"]]);
    pretty_print(&matrix_two);
    println!("");

    // Vec<Vec<i32>>
    let matrix_three =  transpose(vec![vec![1, 2], vec![3, 4]]);
    pretty_print(&matrix_three);
    println!("");

    
}