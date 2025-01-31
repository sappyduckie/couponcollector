use itertools::Itertools;
use std::error::Error;

#[derive(Debug)]
struct CouponError(&'static str);

impl std::fmt::Display for CouponError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CouponError {}

fn comb_pure(g: &[f64], k: usize) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    if k > g.len() {
        return Err(Box::new(CouponError("k > length(g)")));
    }

    if k == 0 {
        return Ok(vec![vec![]]);
    } else if g.len() == 1 {
        return Ok(vec![g.to_vec()]);
    }
    
    Ok(g.iter().cloned().combinations(k).collect())
}

fn comb_ge(g: &[f64], k: usize) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    if k > g.len() {
        return Err(Box::new(CouponError("k > length(g)")));
    }

    let mut result = Vec::new();
    for j in k..=g.len() {
        result.extend(comb_pure(g, j)?);
    }
    Ok(result)
}

fn coup_exact(p: &[f64], n: usize, g: &[usize]) -> Result<f64, Box<dyn Error>> {
    if n < g.len() {
        return Ok(0.0);
    }

    let mut q = 0.0;
    for h in comb_ge(&g.iter().map(|&i| p[i-1]).collect::<Vec<f64>>(), 0)? {
        q += (-1.0_f64).powi((g.len() + h.len()) as i32) * h.iter().sum::<f64>().powi(n as i32);
    }
    Ok(q)
}

fn coup(p: &[f64], n: usize, k: usize, g: &[usize]) -> Result<f64, Box<dyn Error>> {
    if n == 0 {
        return Err(Box::new(CouponError("Number of trials must be positive")));
    }
    if k == 0 || k > g.len() {
        return Err(Box::new(CouponError("Invalid k value")));
    }

    let all_indices: Vec<usize> = (1..=p.len()).collect();
    let complement: Vec<usize> = all_indices.iter()
        .filter(|x| !g.contains(x))
        .cloned()
        .collect();

    let mut q = 0.0;
    for h in comb_ge(g, k)? {
        for hc in comb_ge(&complement, 0)? {
            let mut combined = h.iter()
                .chain(hc.iter())
                .cloned()
                .collect::<Vec<usize>>();
            combined.sort_unstable();
            q += coup_exact(p, n, &combined)?;
        }
    }
    Ok(q)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Example usage
    let p = vec![0.2, 0.2, 0.2, 0.2, 0.2];
    let n = 10;
    let k = 2;
    let g = vec![1, 2, 3];
    
    match coup(&p, n, k, &g) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}