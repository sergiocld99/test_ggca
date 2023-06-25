use std::time::Instant;

use ggca::adjustment::AdjustmentMethod;
use ggca::analysis::Analysis;
use ggca::correlation::CorrelationMethod;
use pyo3::prelude::*;


fn main() -> PyResult<()> {
    println!("Hello, world!");
    
    // File's paths
    let gene_file_path = "mrna.csv".to_string();
    let gem_file_path = "mirna.csv".to_string();

    // Some parameters
    let gem_contains_cpg = false;
    let is_all_vs_all = true;
    let keep_top_n = Some(10); // Keeps the top 10 of correlation (sorting by abs values)
    let collect_gem_dataset = None; // Better performance. Keep small GEM files in memory

    let now = Instant::now();

    let analysis = Analysis {
        gene_file_path, 
        gem_file_path, 
        gem_contains_cpg,
        correlation_method: CorrelationMethod::Pearson,
        correlation_threshold: 0.7, 
        sort_buf_size: 2_000_000,
        adjustment_method: AdjustmentMethod::BenjaminiHochberg,
        is_all_vs_all,
        collect_gem_dataset,
        keep_top_n
    };

    let (result, _total_comb_count, num_comb_evaluated) = 
        analysis.compute()?;

    let seconds = now.elapsed().as_secs();

    for cor_p_value in result.iter() {
	    println!("{}", cor_p_value);
    }

    println!("Terminado en -> {} segundos", seconds);
    println!("Elementos -> {} de {} combinaciones evaluadas", result.len(), num_comb_evaluated);

    Ok(())
}
