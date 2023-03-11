use rand::Rng;
fn main()
{
    const ROW:usize = 4;
    const COLUMN: usize = 6;

    let mut matrix = [[0;COLUMN];ROW];
    for i in 0..ROW
    {
        for j in 0..COLUMN
        {
            matrix[i][j]= rand::thread_rng().gen_range(0..100);
        }
    }
    println!("BEFORE TRANSPOSE");
    for r in 0..matrix.len()
    {
        for c in 0..matrix[r].len()
        {
            print!("{} ",matrix[r][c]);
        }
        println!("");
    }

    let mut transposed_matrix = [[0;ROW];COLUMN];
    for r in 0..COLUMN
    {
        for c in 0..ROW
        {
            transposed_matrix[r][c] = matrix[c][r];
        }
    }
    println!("\nAFTER TRANSPOSE");
    for r in 0..transposed_matrix.len()
    {
        for c in 0..transposed_matrix[r].len()
        {
            print!("{} ",transposed_matrix[r][c]);
        }
        println!("");
    }
}