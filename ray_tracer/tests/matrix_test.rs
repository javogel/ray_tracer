use ray_tracer::matrix::*;

#[test]
fn test_matrix() {
    let width = 2;
    let height = 24;
    let Matrix {data} = matrix(width, height);
    
    assert_eq!(data.len(), height);
    assert_eq!(data[0].len(), width);
}


