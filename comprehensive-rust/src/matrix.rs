// Transposes the matrix
pub fn transpose(matrix: [[u32; 3]; 3]) -> [[u32; 3]; 3] {
    let mut result = [[0; 3]; 3]; // another way to initialize arrays
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}