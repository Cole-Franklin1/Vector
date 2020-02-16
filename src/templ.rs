pub fn det(mat: MatrixND)-> f32{
    let length = mat.len();
    asserteq!(length, mat[0].len());
    for i in 1..length{
        asserteq!(mat[0].len(),mat[i as usize].len());
    }
    let mut deter = 0;
    if length == 2{
        deter += mat[0][0]*mat[1][1];
        deter -= mat[0][1]*mat[1][0];
    }
    else if length == 1{
        mat[0][0]
    }
    else{
        let mut sign = 1;
        for i in 0..length{
            let mut minor = MatrixND::new(length,length);
            let mut row = 0;
            for j in 0..length{
                let mut col = 0;
                for k in 0..length{
                    if j != i && k != i{
                        minor[row][col] = mat[j][k];
                        col += 1;
                    }
                }
                row += 1;
            }
            det += mat[0][i]*det(minor)*sign;
            sign*= -1;
        }
    }
}

impl indexing for VectorND{

}

impl indexing for MatrixND{

}

impl VectorND{
    pub fn len(&self)->usize{
        components.len()
    }
}

impl MatrixND{
    pub fn len(&self)->usize{
        rows.len()
    }
}